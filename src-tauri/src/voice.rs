use crate::provider::TTSBackend;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Voice {
    pub provider: TTSBackend,
    pub id: String,
    pub name: String,
    pub lang: Option<String>,

    // rate, as decimal
    pub rate: f64,

    // pitch, as semitones
    pub pitch: i32,
}
