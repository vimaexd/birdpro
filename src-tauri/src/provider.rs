use crate::voice::Voice;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum TTSBackend {
    #[default]
    MsEdge,
    ElevenLabs,
    Tiktok,

    #[cfg(windows)]
    Windows,
}

#[derive(Clone, Copy, Serialize)]
pub struct TTSBackendInfo {
    pub id: TTSBackend,
    pub name: &'static str,
    pub supported_platforms: &'static [TTSProviderPlatform],
    pub cloud: bool,
    pub uses_credits: bool,
    pub supported_features: &'static [TTSFeature],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TTSFeature {
    Rate,
    Pitch,
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

    AuthorizationRequired, // provider needs api key
    AuthorizationInvalid,  // api key is invalid or expired

    OutOfCredits,
}

impl fmt::Display for TTSBackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TTSBackendError::VoiceNotFound => write!(f, "Requested voice was not found"),
            TTSBackendError::FetchError => write!(
                f,
                "Couldn't connect to server - check your internet connection!"
            ),
            TTSBackendError::SynthesisFailure => write!(f, "Failed to synthesize text to speech"),
            TTSBackendError::DecodeError => write!(f, "Failed to decode TTS audio"),
            TTSBackendError::AuthorizationRequired => write!(
                f,
                "You need to specify an API key in Settings to use this provider"
            ),
            TTSBackendError::AuthorizationInvalid => write!(f, "The API key provided is invalid"),
            TTSBackendError::OutOfCredits => write!(f, "You're out of credits! :("),
        }
    }
}

pub static TTS_BACKENDS: &[TTSBackendInfo] = &[
    TTSBackendInfo {
        id: TTSBackend::MsEdge,
        name: "Microsoft Edge TTS",
        supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux],
        cloud: true,
        uses_credits: false,
        supported_features: &[TTSFeature::Pitch, TTSFeature::Rate],
    },
    TTSBackendInfo {
        id: TTSBackend::ElevenLabs,
        name: "ElevenLabs",
        supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux],
        cloud: true,
        uses_credits: true,
        supported_features: &[TTSFeature::Rate],
    },

    // disabled for now due to not working
    // see tiktok.rs for more details

    // TTSBackendInfo {
    //     id: TTSBackend::Tiktok,
    //     name: "TikTok",
    //     supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux],
    //     cloud: true,
    //     uses_credits: false,
    //     supported_features: &[],
    // },
    #[cfg(windows)]
    TTSBackendInfo {
        id: TTSBackend::Windows,
        name: "Windows",
        supported_platforms: &[TTSProviderPlatform::Windows],
        cloud: false,
        uses_credits: false,
        supported_features: &[TTSFeature::Pitch, TTSFeature::Rate],
    },
];

#[derive(Clone, Copy, Serialize, PartialEq)]
pub enum TTSProviderPlatform {
    Linux,
    Windows,
    Unknown,
    // maybe in the future?
    // MacOS,
}

pub trait TTSProvider {
    fn name() -> &'static str;

    #[allow(async_fn_in_trait)]
    async fn get_speech_bytes(
        message: &str,
        voice: &Voice,
        config: &Value,
    ) -> Result<Vec<u8>, TTSBackendError>;
    #[allow(async_fn_in_trait)]
    async fn get_voices(config: &Value) -> Result<Vec<Voice>, TTSBackendError>;
    fn get_default_voice() -> Voice;
}
