use crate::voice::{Voice, VoiceWithSettings};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum TTSProviderType {
    #[default]
    MsEdge,
    ElevenLabs,
    Tiktok,
    Piper,

    #[cfg(windows)]
    Windows,
}

#[derive(Clone, Copy, Serialize)]
pub struct TTSProviderInfo {
    pub id: TTSProviderType,
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
pub enum TTSProviderError {
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

    MissingVoicesPath
}

impl fmt::Display for TTSProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TTSProviderError::VoiceNotFound => write!(f, "Requested voice was not found"),
            TTSProviderError::FetchError => write!(
                f,
                "Couldn't connect to server - check your internet connection!"
            ),
            TTSProviderError::SynthesisFailure => write!(f, "Failed to synthesize text to speech"),
            TTSProviderError::DecodeError => write!(f, "Failed to decode TTS audio"),
            TTSProviderError::AuthorizationRequired => write!(
                f,
                "You need to specify an API key in Settings to use this provider"
            ),
            TTSProviderError::AuthorizationInvalid => write!(f, "The API key provided is invalid"),
            TTSProviderError::OutOfCredits => write!(f, "You're out of credits! :("),
            TTSProviderError::MissingVoicesPath => write!(f, "You need to specify a voices path in Settings to use this provider"),
        }
    }
}

pub static TTS_PROVIDERS: &[TTSProviderInfo] = &[
    TTSProviderInfo {
        id: TTSProviderType::MsEdge,
        name: "Microsoft Edge TTS",
        supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux],
        cloud: true,
        uses_credits: false,
        supported_features: &[TTSFeature::Pitch, TTSFeature::Rate],
    },
    TTSProviderInfo {
        id: TTSProviderType::ElevenLabs,
        name: "ElevenLabs",
        supported_platforms: &[TTSProviderPlatform::Windows, TTSProviderPlatform::Linux],
        cloud: true,
        uses_credits: true,
        supported_features: &[TTSFeature::Rate],
    },
    TTSProviderInfo {
        id: TTSProviderType::Piper,
        name: "Piper",
        supported_platforms: &[TTSProviderPlatform::Linux],
        cloud: false,
        uses_credits: false,
        supported_features: &[],
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
    TTSProviderInfo {
        id: TTSProviderType::Windows,
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
        voice: &VoiceWithSettings,
        config: &Value,
    ) -> Result<Vec<u8>, TTSProviderError>;
    #[allow(async_fn_in_trait)]
    async fn get_voices(config: &Value) -> Result<Vec<Voice>, TTSProviderError>;
    fn get_default_voice() -> Voice;
}
