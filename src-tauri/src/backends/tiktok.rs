use base64::{Engine, prelude::BASE64_STANDARD};
use serde_json::Value;
use tauri::Manager;
use tauri::path::BaseDirectory;
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;

use crate::{ipc::tts::APP_HANDLE, provider::{TTSBackend, TTSProvider, TTSBackendError}, voice::Voice};

static TIKTOK_BASE_URL: &'static str = "https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/";
static TIKTOK_UA: &'static str = "com.zhiliaoapp.musically/2022600030 (Linux; U; Android 7.1.2; es_ES; SM-G988N; Build/NRD90M;tt-ok/3.12.13.1)";

#[derive(Serialize, Deserialize)]
struct TiktokVoice {
    voice_id: String,
    name: String
}

pub struct TiktokTTSProvider {}

impl TTSProvider for TiktokTTSProvider {
    fn name() -> &'static str {
        "TikTok"
    }

    async fn get_speech_bytes(
            message: &str,
            voice: &crate::voice::Voice,
            _config: &serde_json::Value,
        ) -> Result<Vec<u8>, crate::provider::TTSBackendError> {
        let client = reqwest::Client::new();

        let normalised_message = message
            .replace("+", "plus")
            .replace("&", "and")
            .replace(" ", "+");

        println!("{}", normalised_message);

        // TODO: fix tiktok tts
        // currently it just returns an error no matter what, and
        // i really don't want to just go through Some Guy's random server
        // (i'm sure Some Guy is very nice)

        let _req = client.post(TIKTOK_BASE_URL.to_string() + "?text_speaker=" + voice.id.as_str() + "&req_text=" + normalised_message.as_str() + "&speaker_map_type=0&aid=1233")
            .header("User-Agent", TIKTOK_UA)
            .send()
            .await
            .map_err(|_| TTSBackendError::FetchError)
            .unwrap();

        let status = _req.status();
        match status {
            StatusCode::UNAUTHORIZED => return Err(TTSBackendError::AuthorizationInvalid),
            _ if status != StatusCode::OK => {
                println!("{}", _req.text().await.unwrap());
                return Err(TTSBackendError::FetchError);
            }
            _ => {}
        }

        let json: Value = _req.json().await.unwrap();
        println!("{:?}", json);
        let b64d = json["b64d"].as_str().unwrap();

        BASE64_STANDARD.decode(b64d)
            .map_err(|_| TTSBackendError::DecodeError)
    }

    async fn get_voices(_config: &serde_json::Value) -> Result<Vec<crate::voice::Voice>, crate::provider::TTSBackendError> {
        let voices_path = APP_HANDLE.get().unwrap()
            .path()
            .resolve("assets/data/tiktokVoices.json", BaseDirectory::Resource)
            .unwrap();

        let voices_file = std::fs::File::open(&voices_path).unwrap();
        let voices_json: Vec<TiktokVoice> = serde_json::from_reader(voices_file).unwrap();

        Ok(
            voices_json.iter()
                .map(|v| {
                    Voice {
                        id: v.voice_id.clone(),
                        name: v.name.clone(),
                        lang: None,
                        pitch: 0,
                        rate: 1.0,
                        provider: TTSBackend::Tiktok
                    }
                }).collect()
        )
    }

    fn get_default_voice() -> Voice {
        Voice {
            provider: TTSBackend::Tiktok,
            id: "en_us_002".to_string(),
            name: "Jessie [Female]".to_string(),
            lang: None,
            pitch: 0,
            rate: 1.0,
        }
    }
}
