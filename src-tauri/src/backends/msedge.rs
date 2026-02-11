use crate::provider::{TTSBackend, TTSBackendError, TTSProvider};
use crate::voice::Voice;
use msedge_tts::tts::client::connect_async;
use msedge_tts::tts::SpeechConfig;
use msedge_tts::voice::get_voices_list;
use serde_json::Value;

pub struct MsEdgeTTSProvider {}

impl TTSProvider for MsEdgeTTSProvider {
    fn name() -> &'static str {
        "Microsoft Edge TTS"
    }

    async fn get_speech_bytes(
        message: &str,
        voice: &Voice,
        config: &Value,
    ) -> Result<Vec<u8>, TTSBackendError> {
        let voices = get_voices_list().unwrap();

        let _resolved_voice = voices.iter().find(|x| &x.name == &voice.id);

        if _resolved_voice.is_none() {
            return Err(TTSBackendError::VoiceNotFound);
        }

        let resolved_voice = _resolved_voice.unwrap();

        let mut speech_config = SpeechConfig::from(resolved_voice);
        speech_config.pitch = voice.pitch.into();
        speech_config.rate = (voice.rate * 10.0).round() as i32;

        let mut tts = connect_async()
            .await
            .map_err(|_| TTSBackendError::FetchError)?;

        let audio = tts
            .synthesize(message, &speech_config)
            .await
            .map_err(|_| TTSBackendError::FetchError)?;

        Ok(audio.audio_bytes)
    }

    async fn get_voices(config: &Value) -> Result<Vec<Voice>, TTSBackendError> {
        let voices = match get_voices_list() {
            Ok(v) => v,
            Err(_) => {
                log::error!("failed to fetch voices list");
                return Ok(vec![]);
            }
        };
        let mapped: Vec<Voice> = voices
            .iter()
            .map(|x| Voice {
                provider: TTSBackend::MsEdge,
                id: x.name.clone(),
                name: x.friendly_name.clone().unwrap(),
                lang: x.locale.clone(),
                pitch: 0,
                rate: 1.0,
            })
            .collect();

        Ok(mapped)
    }

    fn get_default_voice() -> Voice {
        return Voice {
            provider: TTSBackend::MsEdge,
            id: "Microsoft Server Speech Text to Speech Voice (en-US, EmmaNeural)".to_string(),
            name: "Microsoft Emma Online (Natural) - English (United States)".to_string(),
            lang: Some("en-US".to_string()),
            pitch: 0,
            rate: 1.0,
        };
    }
}
