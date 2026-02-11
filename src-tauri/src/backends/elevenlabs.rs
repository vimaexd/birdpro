
use std::collections::HashMap;

use serde_json::Value;

use crate::provider::{TTSBackend, TTSBackendError, TTSProvider};
use crate::voice::Voice;

static ELEVENLABS_BASE_URL: &'static str = "https://api.elevenlabs.io";

pub struct ElevenlabsTTSProvider {}

impl TTSProvider for ElevenlabsTTSProvider {
    fn name() -> &'static str {
        "ElevenLabs"
    }

    async fn get_speech_bytes(message: &str, voice: &Voice, config: &Value) -> Result<Vec<u8>, TTSBackendError> {
        let apikey = match config["elevenlabs.apikey"].as_str() {
            Some(a) => a,
            None => {
                return Err(TTSBackendError::AuthorizationRequired);
            }
        };

        if apikey == "" {
            return Err(TTSBackendError::AuthorizationRequired);
        }

        let client = reqwest::Client::new();
        let mut body = HashMap::new();
        body.insert("text", message);
        body.insert("model", "eleven_flash_v2_5");

        let _req = client.post(ELEVENLABS_BASE_URL.to_owned() + "/v1/text-to-speech/" + voice.id.as_str())
            .json(&body)
            .header("xi-api-key", apikey)
            .send()
            .await.map_err(|_| TTSBackendError::FetchError)
            .unwrap();

        let req = _req.error_for_status()
            .map_err(|e| {
                TTSBackendError::FetchError
            })?;

        let bytes = req.bytes().await.map_err(|_| TTSBackendError::DecodeError).unwrap();
        Ok(Vec::from(bytes))
    }

    async fn get_voices(config: &Value) -> Result<Vec<Voice>, TTSBackendError> {
        let apikey = match config["elevenlabs.apikey"].as_str() {
            Some(a) => a,
            None => {
                return Err(TTSBackendError::AuthorizationRequired);
            }
        };

        if apikey == "" {
            return Err(TTSBackendError::AuthorizationRequired);
        }

        let client = reqwest::Client::new();
        let _req = client.get(ELEVENLABS_BASE_URL.to_owned() + "/v2/voices?page_size=100")
            .header("xi-api-key", apikey)
            .send()
            .await.map_err(|_| TTSBackendError::FetchError)
            .unwrap();

        let j = _req.json::<Value>().await.unwrap();

        let voices: Vec<Value> = j["voices"].as_array().unwrap().clone();

        voices.into_iter().map(|v|
            Ok(Voice {
                provider: TTSBackend::ElevenLabs,
                id: v["voice_id"].as_str().unwrap().to_owned(),
                name: v["name"].as_str().unwrap().to_owned(),
                lang: v["language"].as_str().map(str::to_owned),
                pitch: 0,
                rate: 0.0
            })
        ).collect()
    }

    fn get_default_voice() -> Voice {
        Voice {
            provider: TTSBackend::ElevenLabs,
            id: "".to_string(),
            name: "Test voice".to_string(),
            lang: None,
            rate: 0.0,
            pitch: 0,
        }
    }
}
