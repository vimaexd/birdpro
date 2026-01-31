use msedge_tts::voice::{get_voices_list};
use msedge_tts::tts::SpeechConfig;
use msedge_tts::tts::client::connect_async;
use crate::provider::{TTSProvider, TTSProviderPlatform};

// the voice names are so long so i'm going to manually truncate them
static EDGE_VOICE_PREFIX: &'static str = "Microsoft Server Speech Text to Speech Voice (";

pub struct MsEdgeTTSProvider {}

impl TTSProvider for MsEdgeTTSProvider {
    fn name() -> &'static str {
        "Microsoft Edge TTS"
    }

    async fn get_speech_bytes(message: &str, voice: &String) -> Result<Vec<u8>, ()> {
        let voices = get_voices_list().unwrap();

        let voice = voices.iter().find(|x| {
            &x.name == &(EDGE_VOICE_PREFIX.to_owned() + voice + ")")
        }).expect("voice not found");

        let speech_config = SpeechConfig::from(voice);
        let mut tts = connect_async().await.unwrap();
        let audio = tts
            .synthesize(message, &speech_config)
            .await
            .unwrap();
        // format is usually audio-24khz-48kbitrate-mono-mp3

        Ok(audio.audio_bytes)
    }

    // TODO refactor this to use a Voice struct instead of string
    fn get_voices() -> Vec<String> {
        let voices = get_voices_list().expect("voices list failed");
        let mapped: Vec<String> = voices.iter().map(|x| {
            // remove prefix
            let mut v = x.clone().name.replace(EDGE_VOICE_PREFIX, "");

            // remove ending )
            v.pop();

            v
        }).collect();

        mapped
    }

    fn get_default_voice() -> String {
        return "en-GB, LibbyNeural".to_string();
    }
}
