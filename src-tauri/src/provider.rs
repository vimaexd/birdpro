
#[derive(Default, Clone, Copy, serde::Serialize)]
pub enum TTSBackend {
    #[default]
    MsEdge
}

#[derive(Clone, Copy, serde::Serialize)]
pub struct TTSBackendInfo {
    id: TTSBackend,
    name: &'static str,
}

pub static TTS_BACKENDS: &[TTSBackendInfo] = &[
    TTSBackendInfo {
        id: TTSBackend::MsEdge,
        name: "Microsoft Edge TTS"
    },
];

pub trait TTSProvider {
    fn name() -> &'static str;

    async fn get_speech_bytes(message: &str, voice: &String) -> Result<Vec<u8>, ()>;
    fn get_voices() -> Vec<String>;
    fn get_default_voice() -> String;
}
