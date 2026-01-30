use msedge_tts::voice::{get_voices_list};
use msedge_tts::tts::SpeechConfig;
use msedge_tts::tts::client::connect_async;
use crate::provider::TTSProvider;

pub struct MsEdgeTTSProvider {}

impl TTSProvider for MsEdgeTTSProvider {
    fn name() -> &'static str {
        "Microsoft Edge Read-Aloud"
    }

    async fn get_speech_bytes(message: &str, voice: &String) -> Result<Vec<u8>, ()> {
        let voices = get_voices_list().unwrap();
        let voice = voices.iter().find(|x| {
            &x.name == voice
        }).expect("voice not found");

        let speech_config = SpeechConfig::from(voice);
        let mut tts = connect_async().await.unwrap();
        let audio = tts
            .synthesize(message, &speech_config)
            .await
            .unwrap();

        println!("got speech wave");

        // in testing this was usually audio-24khz-48kbitrate-mono-mp3
        println!("format: {}", audio.audio_format);

        Ok(audio.audio_bytes)
    }

    // TODO refactor this to use a Voice struct instead of string
    fn get_voices() -> Vec<String> {
        let voices = get_voices_list().expect("voices list failed");
        let mapped: Vec<String> = voices.iter().map(|x| {
            x.clone().name
        }).collect();

        mapped
    }

    fn get_default_voice() -> String {
        // let voices = get_voices_list().expect("voices list failed");
        // voices[0].name.clone()
        //
        return "Microsoft Server Speech Text to Speech Voice (en-GB, LibbyNeural)".to_string();
    }
}
