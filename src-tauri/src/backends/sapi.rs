use msedge_tts::voice::{get_voices_list};
use msedge_tts::tts::SpeechConfig;
use msedge_tts::tts::client::connect_async;
use crate::provider::{TTSProvider, TTSProviderPlatform};

#[cfg(windows)]
use sapi_lite::tts::installed_voices;

pub struct WindowsTTSProvider {}

#[cfg(windows)]
impl TTSProvider for WindowsTTSProvider {
    fn name() -> &'static str {
        "Windows"
    }

    async fn get_speech_bytes(message: &str, voice: &String) -> Result<Vec<u8>, ()> {
        sapi_lite::initialize().unwrap();
        let synth = SyncSynthesizer::new().unwrap();


        sapi_lite::finalize();

        Ok(vec![])
    }

    // TODO refactor this to use a Voice struct instead of string
    fn get_voices() -> Vec<String> {
        let voices = installed_voices(None, None).unwrap();
        voices.map(|v| {
            v.name()
        }).collect()
    }

    fn get_default_voice() -> String {
        let mut voices = installed_voices(None, None).unwrap();
        voices.nth(0)?.name()?.into_string().unwrap()
    }
}
