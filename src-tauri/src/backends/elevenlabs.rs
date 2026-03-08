use reqwest::StatusCode;
use serde_json::Value;

use crate::provider::{TTSProvider, TTSProviderError, TTSProviderType};
use crate::voice::{Voice, VoiceWithSettings};

static ELEVENLABS_BASE_URL: &'static str = "https://api.elevenlabs.io";

pub struct ElevenlabsTTSProvider {}

impl TTSProvider for ElevenlabsTTSProvider {
    fn name() -> &'static str {
        "ElevenLabs"
    }

    async fn get_speech_bytes(
        message: &str,
        voice: &VoiceWithSettings,
        config: &Value,
    ) -> Result<Vec<u8>, TTSProviderError> {
        let apikey = match config["elevenlabs.apikey"].as_str() {
            Some(a) => a,
            None => {
                return Err(TTSProviderError::AuthorizationRequired);
            }
        };

        if apikey == "" {
            return Err(TTSProviderError::AuthorizationRequired);
        }

        let client = reqwest::Client::new();

        // elevenlabs speed only goes from 0.7 to 1.2
        let speed_mapped = 0.7 + ((voice.rate + 8.0) / 16.0) * 0.5;

        let body = serde_json::json!({
            "text": message,
            "model_id": "eleven_flash_v2_5",
            "voice_settings": {
                "speed": speed_mapped,
            }
        });

        let _req = client
            .post(ELEVENLABS_BASE_URL.to_owned() + "/v1/text-to-speech/" + voice.voice.id.as_str())
            .json(&body)
            .header("xi-api-key", apikey)
            .send()
            .await
            .map_err(|_| TTSProviderError::FetchError)
            .unwrap();

        let status = _req.status();
        match status {
            StatusCode::UNAUTHORIZED => return Err(TTSProviderError::AuthorizationInvalid),
            StatusCode::PAYMENT_REQUIRED => return Err(TTSProviderError::OutOfCredits),
            _ if status != StatusCode::OK => {
                println!("{}", _req.text().await.unwrap());
                return Err(TTSProviderError::FetchError);
            }
            _ => {}
        }

        let bytes = _req
            .bytes()
            .await
            .map_err(|_| TTSProviderError::DecodeError)
            .unwrap();
        Ok(Vec::from(bytes))
    }

    async fn get_voices(config: &Value) -> Result<Vec<Voice>, TTSProviderError> {
        let apikey = match config["elevenlabs.apikey"].as_str() {
            Some(a) => a,
            None => {
                return Err(TTSProviderError::AuthorizationRequired);
            }
        };

        if apikey == "" {
            return Err(TTSProviderError::AuthorizationRequired);
        }

        let client = reqwest::Client::new();
        let _req = client
            .get(ELEVENLABS_BASE_URL.to_owned() + "/v2/voices?page_size=100")
            .header("xi-api-key", apikey)
            .send()
            .await
            .map_err(|_| TTSProviderError::FetchError)
            .unwrap();

        match _req.status() {
            StatusCode::UNAUTHORIZED => return Err(TTSProviderError::AuthorizationInvalid),
            _ => {}
        }

        let j = _req.json::<Value>().await.unwrap();

        let voices: Vec<Value> = j["voices"].as_array().unwrap().clone();

        voices
            .into_iter()
            .map(|v| {
                Ok(Voice {
                    provider: TTSProviderType::ElevenLabs,
                    id: v["voice_id"].as_str().unwrap().to_owned(),
                    name: v["name"].as_str().unwrap().to_owned(),
                    lang: v["language"].as_str().map(str::to_owned),
                })
            })
            .collect()
    }

    fn get_default_voice() -> Voice {
        Voice {
            provider: TTSProviderType::ElevenLabs,
            id: "".to_string(),
            name: "Test voice".to_string(),
            lang: None,
        }
    }
}
