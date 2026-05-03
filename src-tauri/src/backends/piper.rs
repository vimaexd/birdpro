use crate::provider::TTSProvider;
use crate::provider::TTSProviderError;
use crate::provider::TTSProviderType;
use crate::voice::Voice;
use piper_rs::synth::PiperSpeechSynthesizer;
use serde_json::Value;
use std::io::Cursor;
use std::path::Path;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use walkdir::WalkDir;

struct PiperSpeechSynthWrapper(PiperSpeechSynthesizer);
impl std::fmt::Debug for PiperSpeechSynthWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PiperSpeechSynthesizer")
    }
}

static PIPER_VOICE_CACHE: LazyLock<Mutex<Option<(String, PiperSpeechSynthWrapper, Value)>>> =
    LazyLock::new(|| Mutex::new(None));

pub struct PiperTTSProvider {}

impl TTSProvider for PiperTTSProvider {
    fn name() -> &'static str {
        "Piper"
    }

    async fn get_speech_bytes(
        message: &str,
        voice: &crate::voice::VoiceWithSettings,
        _config: &serde_json::Value,
    ) -> Result<Vec<u8>, crate::provider::TTSProviderError> {
        let mut cache = PIPER_VOICE_CACHE.lock().await;

        if cache.is_none() || (voice.voice.id.to_string() != cache.as_ref().unwrap().0) {
            let model = piper_rs::from_config_path(Path::new(voice.voice.id.as_str()))
                .map_err(|_| TTSProviderError::VoiceNotFound)
                .unwrap();

            let file = std::fs::File::open(voice.voice.id.as_str()).unwrap();
            let json: Value = serde_json::from_reader(file)
                .map_err(|_| TTSProviderError::VoiceNotFound)
                .unwrap();

            let synth = PiperSpeechSynthesizer::new(model)
                .map_err(|_| TTSProviderError::VoiceNotFound)
                .unwrap();

            *cache = Some((
                voice.voice.id.to_string(),
                // TODO: handle errors
                PiperSpeechSynthWrapper(synth),
                json,
            ));
        }

        let synth = &cache.as_ref().unwrap().1 .0;
        let mut samples: Vec<f32> = Vec::new();
        let audio = synth
            .synthesize_parallel(message.to_string(), None)
            .map_err(|_| TTSProviderError::SynthesisFailure)
            .unwrap();
        for result in audio {
            samples.append(&mut result.unwrap().into_vec());
        }

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: cache.as_ref().unwrap().2["audio"]["sample_rate"]
                .as_u64()
                .unwrap() as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut buf = Vec::new();
        {
            let mut writer = hound::WavWriter::new(Cursor::new(&mut buf), spec).unwrap();
            for sample in &samples {
                writer.write_sample(*sample).unwrap();
            }
            writer.finalize().unwrap();
        }

        Ok(buf)
    }

    async fn get_voices(
        config: &serde_json::Value,
    ) -> Result<Vec<crate::voice::Voice>, crate::provider::TTSProviderError> {
        let voices_path = config["piper.voicesPath"].as_str();

        if voices_path.is_none() || voices_path.unwrap().is_empty() {
            return Err(TTSProviderError::MissingVoicesPath);
        }

        let mut voices = vec![];

        for entry in WalkDir::new(voices_path.unwrap())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file()
                && entry.file_name().to_str().unwrap().ends_with(".onnx.json")
            {
                let file = std::fs::File::open(entry.path()).unwrap();
                let json: Value = match serde_json::from_reader(file) {
                    Ok(v) => v,
                    Err(_) => {
                        continue;
                    }
                };

                let mut name = format!(
                    "{}",
                    json["dataset"]
                        .as_str()
                        .unwrap_or_else(|| { entry.file_name().to_str().unwrap() })
                        .to_string()
                );

                let quality = json["audio"]["quality"].as_str();
                if quality.is_some() {
                    name = name + " " + format!("({})", quality.unwrap()).as_str();
                }

                voices.push(Voice {
                    provider: TTSProviderType::Piper,
                    id: entry.path().to_string_lossy().to_string(),
                    name: name,
                    lang: json["language"]["code"].as_str().map(|o| o.to_string()),
                });
            }
        }

        Ok(voices)
    }

    fn get_default_voice() -> crate::voice::Voice {
        Voice {
            provider: TTSProviderType::Piper,
            id: "test".to_string(),
            name: "test".to_string(),
            lang: None,
        }
    }
}
