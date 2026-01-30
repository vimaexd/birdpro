#[derive(Default)]
pub enum TTSBackend {
    #[default]
    MsEdge
}

pub trait TTSProvider {
    fn get_speech_bytes(message: &str, voice: &String) -> Vec<u8>;
    fn get_voices() -> Vec<String>;
    fn get_default_voice() -> String;
}

// TODO: impl https://crates.io/crates/msedge-tts
