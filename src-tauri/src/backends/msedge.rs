use msedge_tts::voice::{get_voices_list};
use msedge_tts::tts::SpeechConfig;
use msedge_tts::tts::client::connect_async;
use crate::voice::Voice;
use crate::provider::{self, TTSProvider, TTSProviderPlatform, TTSBackend};

// the voice names are so long so i'm going to manually truncate them
static EDGE_VOICE_PREFIX: &'static str = "Microsoft Server Speech Text to Speech Voice (";

pub struct MsEdgeTTSProvider {}

impl TTSProvider for MsEdgeTTSProvider {
    fn name() -> &'static str {
        "Microsoft Edge TTS"
    }

    async fn get_speech_bytes(message: &str, voice: &Voice) -> Result<Vec<u8>, ()> {
        let voices = get_voices_list().unwrap();

        // let resolved_voice = voices.iter().find(|x| {
        //     &x.name == &(EDGE_VOICE_PREFIX.to_owned() + voice + ")")
        // }).expect("voice not found");

        let resolved_voice = voices.iter().find(|x| {
            &x.name == &voice.id
        }).expect("voice not found");

        let mut speech_config = SpeechConfig::from(resolved_voice);
        speech_config.pitch = voice.pitch.into();
        speech_config.rate = (voice.rate * 10.0).round() as i32;

        let mut tts = connect_async().await.unwrap();
        let audio = tts
            .synthesize(message, &speech_config)
            .await
            .unwrap();
        // format is usually audio-24khz-48kbitrate-mono-mp3

        Ok(audio.audio_bytes)
    }

    // TODO refactor this to use a Voice struct instead of string
    fn get_voices() -> Vec<Voice> {
        let voices = match get_voices_list() {
            Ok(v) => v,
            Err(_) => {
                log::error!("[MsEdge] Failed to fetch voices list");
                return vec![];
            }
        };
        let mapped: Vec<Voice> = voices.iter().map(|x| {
            Voice {
                provider: TTSBackend::MsEdge,
                id: x.name.clone(),
                name: x.friendly_name.clone().unwrap(),
                pitch: 0,
                rate: 1.0
            }
        }).collect();

        mapped
    }

    fn get_default_voice() -> Voice {
        return Voice {
            provider: TTSBackend::MsEdge,
            id: "Microsoft Server Speech Text to Speech Voice (en-US, EmmaNeural)".to_string(),
            name: "Microsoft Emma Online (Natural) - English (United States)".to_string(),
            pitch: 0,
            rate: 1.0
        }
    }
}
