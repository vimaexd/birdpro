use serde::{Serialize, Deserialize};
use crate::voice::{Voice};

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum TTSBackend {
    #[default]
    MsEdge,

    // TikTok,

    #[cfg(windows)]
    Windows
}

#[derive(Clone, Copy, Serialize)]
pub struct TTSBackendInfo {
    pub id: TTSBackend,
    pub name: &'static str,
    pub supported_platforms: &'static [TTSProviderPlatform],
    pub cloud: bool
}

pub static TTS_BACKENDS: &[TTSBackendInfo] = &[
    TTSBackendInfo {
        id: TTSBackend::MsEdge,
        name: "Microsoft Edge TTS",
        supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux],
        cloud: true
    },
    // TTSBackendInfo {
    //     id: TTSBackend::TikTok,
    //     name: "TikTok",
    //     supported_platforms: &[],
    //     cloud: true
    // },
    #[cfg(windows)]
    TTSBackendInfo {
        id: TTSBackend::Windows,
        name: "Windows",
        supported_platforms: &[TTSProviderPlatform::Windows],
        cloud: false
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

    #[allow(async_fn_in_trait)]
    async fn get_speech_bytes(message: &str, voice: &Voice) -> Result<Vec<u8>, ()>;
    fn get_voices() -> Vec<Voice>;
    fn get_default_voice() -> Voice;
}
