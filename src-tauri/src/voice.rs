use crate::provider::TTSProviderType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Voice {
    pub provider: TTSProviderType,
    pub id: String,
    pub name: String,
    pub lang: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VoiceWithSettings {
    pub voice: Voice,
    pub rate: f64,  // rate, as decimal
    pub pitch: i32, // pitch, as semitones
}
