use crate::provider::{TTSProvider, TTSProviderError, TTSProviderType};
use crate::voice::{Voice, VoiceWithSettings};
use serde_json::Value;
use windows::core::{Interface, HSTRING};
use windows::Media::SpeechSynthesis::{SpeechSynthesisStream, SpeechSynthesizer};
use windows::Storage::Streams::{Buffer, IBuffer, InputStreamOptions};
use windows::Win32::System::WinRT::IBufferByteAccess;

pub struct WindowsTTSProvider {}

impl TTSProvider for WindowsTTSProvider {
    fn name() -> &'static str {
        "Windows"
    }

    async fn get_speech_bytes(
        message: &str,
        voice: &VoiceWithSettings,
        _config: &Value,
    ) -> Result<Vec<u8>, TTSProviderError> {
        let synth = SpeechSynthesizer::new().unwrap();

        let voices = SpeechSynthesizer::AllVoices().unwrap();
        let resolved_voice = voices
            .into_iter()
            .find(|x| &x.DisplayName().unwrap().to_string() == &voice.voice.name);

        if resolved_voice.is_none() {
            return Err(TTSProviderError::VoiceNotFound);
        }

        synth.SetVoice(&resolved_voice.unwrap()).unwrap();

        let lang = voice.voice.lang.as_ref().unwrap();
        let rate = (voice.rate * 10.0).round() as i32;

        let ssml = format!("<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" xml:lang=\"{lang}\">
                <prosody rate=\"{rate:+}%\">
                    {message}
                </prosody>
        </speak>");

        let speech_stream: SpeechSynthesisStream = synth
            .SynthesizeSsmlToStreamAsync(&HSTRING::from(ssml))
            .expect("failed to synthesize")
            .await
            .map_err(|_| TTSProviderError::SynthesisFailure)
            .unwrap();

        // microsoft I hate you
        let size = speech_stream.Size().unwrap() as u32;
        let mut bytes = Vec::new();

        let buf = Buffer::Create(size).unwrap();
        let ibuf: IBuffer = speech_stream
            .ReadAsync(&buf, size, InputStreamOptions::None)
            .unwrap()
            .await
            .unwrap();

        let len = ibuf.Length().unwrap();
        if len == 0 {
            // no data i guess?
            return Ok(vec![]);
        }

        let byte_access: IBufferByteAccess = ibuf.cast().unwrap();
        let ptr = unsafe { byte_access.Buffer().unwrap() };
        let slic = unsafe { std::slice::from_raw_parts(ptr, len as usize).to_vec() };

        bytes.extend_from_slice(&slic);

        Ok(bytes)
    }

    async fn get_voices(_config: &Value) -> Result<Vec<Voice>, TTSProviderError> {
        let voices = SpeechSynthesizer::AllVoices().unwrap();
        Ok(voices
            .into_iter()
            .map(|x| Voice {
                provider: TTSProviderType::Windows,
                id: x.Id().unwrap().to_string(),
                name: x.DisplayName().unwrap().to_string(),
                lang: Some(x.Language().unwrap().to_string()),
            })
            .collect())
    }

    fn get_default_voice() -> Voice {
        let default = SpeechSynthesizer::DefaultVoice().unwrap();
        Voice {
            provider: TTSProviderType::Windows,
            id: default.Id().unwrap().to_string(),
            name: default.DisplayName().unwrap().to_string(),
            lang: Some(default.Language().unwrap().to_string()),
        }
    }
}
