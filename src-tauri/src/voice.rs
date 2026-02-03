use serde::{Deserialize, Serialize};
use crate::provider::TTSBackend;

#[derive(Serialize, Deserialize, Clone)]
pub struct Voice {
    pub provider: TTSBackend,
    pub id: String,
    pub name: String,

    // rate, as decimal
    pub rate: f64,

    // pitch, as semitones
    pub pitch: i32
}
