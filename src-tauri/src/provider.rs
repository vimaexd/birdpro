use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum TTSBackend {
    #[default]
    MsEdge,

    TikTok,

    #[cfg(windows)]
    Windows
}

#[derive(Clone, Copy, Serialize)]
pub struct TTSBackendInfo {
    pub id: TTSBackend,
    pub name: &'static str,
    pub supported_platforms: &'static [TTSProviderPlatform]
}

pub static TTS_BACKENDS: &[TTSBackendInfo] = &[
    TTSBackendInfo {
        id: TTSBackend::MsEdge,
        name: "Microsoft Edge TTS",
        supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux]
    },
    TTSBackendInfo {
        id: TTSBackend::TikTok,
        name: "TikTok",
        supported_platforms: &[]
    },
    #[cfg(windows)]
    TTSBackendInfo {
        id: TTSBackend::Windows,
        name: "Windows",
        supported_platforms: &[TTSProviderPlatform::Windows]
    },
];

#[derive(Clone, Copy, Serialize, PartialEq)]
pub enum TTSProviderPlatform {
    Linux,
    Windows,
    Unknown

    // maybe in the future?
    // MacOS,
}

pub trait TTSProvider {
    fn name() -> &'static str;

    async fn get_speech_bytes(message: &str, voice: &String) -> Result<Vec<u8>, ()>;
    fn get_voices() -> Vec<String>;
    fn get_default_voice() -> String;
}
