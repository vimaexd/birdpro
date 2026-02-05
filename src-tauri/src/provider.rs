use crate::voice::Voice;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum TTSBackend {
    #[default]
    MsEdge,

    // TikTok,
    #[cfg(windows)]
    Windows,
}

#[derive(Clone, Copy, Serialize)]
pub struct TTSBackendInfo {
    pub id: TTSBackend,
    pub name: &'static str,
    pub supported_platforms: &'static [TTSProviderPlatform],
    pub cloud: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TTSBackendError {
    // voice not found
    VoiceNotFound,

    // internet fetch error
    FetchError,

    // failed to synthesize
    SynthesisFailure,

    // failed to decode audio
    DecodeError,
}

impl fmt::Display for TTSBackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TTSBackendError::VoiceNotFound => write!(f, "Requested voice was not found"),
            TTSBackendError::FetchError => write!(
                f,
                "Couldn't connect to server - check your internet connection!"
            ),
            TTSBackendError::SynthesisFailure => write!(
                f,
                "Failed to synthesize text to speech"
            ),
            TTSBackendError::DecodeError => write!(
                f,
                "Failed to decode TTS audio"
            ),
        }
    }
}

pub static TTS_BACKENDS: &[TTSBackendInfo] = &[
    TTSBackendInfo {
        id: TTSBackend::MsEdge,
        name: "Microsoft Edge TTS",
        supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux],
        cloud: true,
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
        cloud: false,
    },
];

#[derive(Clone, Copy, Serialize, PartialEq)]
pub enum TTSProviderPlatform {
    Linux,
    Windows,
    Unknown, // maybe in the future?
             // MacOS,
}

pub trait TTSProvider {
    fn name() -> &'static str;

    #[allow(async_fn_in_trait)]
    async fn get_speech_bytes(message: &str, voice: &Voice) -> Result<Vec<u8>, TTSBackendError>;
    fn get_voices() -> Result<Vec<Voice>, TTSBackendError>;
    fn get_default_voice() -> Voice;
}
