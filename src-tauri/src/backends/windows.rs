use windows::core::HSTRING;
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Storage::Streams::IBuffer;
use crate::provider::{TTSProvider};

pub struct WindowsTTSProvider {}

impl TTSProvider for WindowsTTSProvider {
    fn name() -> &'static str {
        "Windows"
    }

    async fn get_speech_bytes(message: &str, voice: &String) -> Result<Vec<u8>, ()> {
        let synth = SpeechSynthesizer::new().unwrap();

        let voices = SpeechSynthesizer::AllVoices().unwrap();
        let voice = voices.into_iter().find(|x| {
            &x.DisplayName().unwrap().to_string() == voice
        }).unwrap();

        synth.SetVoice(&voice).unwrap();

        let buf: Vec<u8> = vec![];
        // let stream = synth.SynthesizeTextToStreamAsync(&HSTRING::from(message)).unwrap().await.unwrap().ReadAsync(buf, );


        Ok(vec![])
    }

    fn get_voices() -> Vec<String> {
        let voices = SpeechSynthesizer::AllVoices().unwrap();
        voices.into_iter().map(|x| {
            x.DisplayName().unwrap().to_string()
        }).collect()
    }

    fn get_default_voice() -> String {
        SpeechSynthesizer::DefaultVoice().unwrap().DisplayName().unwrap().to_string()
    }
}
