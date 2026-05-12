use crate::provider::TTSProviderType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Voice {
    pub provider: TTSProviderType,
    pub id: String,
    pub name: String,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VoiceWithSettings {
    pub voice: Voice,
    pub rate: f64,  // rate, as decimal
    pub pitch: i32, // pitch, as semitones
}

#[derive(Clone)]
pub struct CachedMessage {
    pub content: String,
    pub voice: VoiceWithSettings,
    pub audio_bytes: Vec<u8>
}

impl PartialEq for VoiceWithSettings {
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch
        && self.rate == other.rate
        && self.voice.provider == other.voice.provider
        && self.voice.id == other.voice.id
    }
}
