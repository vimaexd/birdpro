use std::collections::HashMap;
use reqwest;
use serde::Serialize;
use serde_json::Value;
use crate::{provider::{TTSProvider}, voice::Voice};

// TODO: fix all of this and make it work with a session id

static TTBASEURL: &'static str = "";
static TTUA: &'static str = "com.zhiliaoapp.musically/2022600030 (Linux; U; Android 7.1.2; es_ES; SM-G988N; Build/NRD90M;tt-ok/3.12.13.1)";

#[derive(Serialize)]
struct TiktokResponse {}


pub struct TiktokTTSProvider {}
impl TTSProvider for TiktokTTSProvider {
    fn name() -> &'static str {
        "TikTok"
    }

    async fn get_speech_bytes(message: &str, voice: &Voice) -> Result<Vec<u8>, ()> {

        let mut map = HashMap::new();
        map.insert("text", message);
        map.insert("voice", voice.id.as_str());

        let client = reqwest::Client::new();
        let resp: Value = client.post(TTBASEURL)
            .header("Content-Type", "application/json")
            .header("User-Agent", TTUA)
            .json(&map)
            .send()
            .await.unwrap()
            .json().await.unwrap();

        println!("{:#}", resp);

        Ok(vec![])
    }

    fn get_voices() -> Vec<Voice> {
        // TODO: write
        vec![Voice {
            provider: crate::provider::TTSBackend::TikTok,
            id: "".to_string(),
            name: "test".to_string(),
            rate: 1.0,
            pitch: 0
        }]
    }

    fn get_default_voice() -> Voice {
        Voice {
            provider: crate::provider::TTSBackend::TikTok,
            id: "".to_string(),
            name: "test".to_string(),
            rate: 1.0,
            pitch: 0
        }
    }
}
