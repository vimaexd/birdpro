use crate::provider::{TTSBackend, TTSBackendError, TTSProvider};
use crate::voice::Voice;
use windows::core::{Interface, HSTRING};
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Storage::Streams::{Buffer, InputStreamOptions};
use windows::Win32::System::WinRT::IBufferByteAccess;

pub struct WindowsTTSProvider {}

impl TTSProvider for WindowsTTSProvider {
    fn name() -> &'static str {
        "Windows"
    }

    async fn get_speech_bytes(message: &str, voice: &Voice) -> Result<Vec<u8>, TTSBackendError> {
        let synth = SpeechSynthesizer::new().unwrap();

        let voices = SpeechSynthesizer::AllVoices().unwrap();
        let resolved_voice = voices
            .into_iter()
            .find(|x| &x.Id().unwrap().to_string() == &voice.id)
            .expect("couldnt resolve system voice");

        synth.SetVoice(&resolved_voice).unwrap();

        let voice_name = voice.name.clone();
        let pitch = voice.pitch;
        let rate = (voice.rate * 10.0).round() as i32;

        let ssml = format!("<speak version=\"1.0\" xmlns=\"http://www.w3.org/2001/10/synthesis\" xml:lang=\"en-US\">
            <voice name=\"{voice_name}\">
                <prosody pitch=\"{pitch:+}%\" rate=\"{rate:+}%\">
                    {message}
                </prosody>
            </voice>
        </speak>");

        let speech_stream = synth
            .SynthesizeSsmlToStreamAsync(&HSTRING::from(ssml))
            .expect("failed to synthesize")
            .await
            .expect("failed to synthesize after async");

        // microsoft i hate you
        let size = speech_stream.Size().unwrap() as u32;
        let mut bytes = Vec::new();

        let buf = Buffer::Create(size).unwrap();
        let ibuf = speech_stream
            .ReadAsync(&buf, size, InputStreamOptions::None)
            .expect("ibuf failed")
            .await
            .expect("ibuf failed post-async");

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

    fn get_voices() -> Result<Vec<Voice>, TTSBackendError> {
        let voices = SpeechSynthesizer::AllVoices().unwrap();
        Ok(
            voices
                .into_iter()
                .map(|x| Voice {
                    provider: TTSBackend::Windows,
                    id: x.Id().unwrap().to_string(),
                    name: x.DisplayName().unwrap().to_string(),
                    rate: 1.0,
                    pitch: 0,
                })
                .collect()
        )
    }

    fn get_default_voice() -> Voice {
        let default = SpeechSynthesizer::DefaultVoice().unwrap();
        Voice {
            provider: TTSBackend::Windows,
            id: default.Id().unwrap().to_string(),
            name: default.DisplayName().unwrap().to_string(),
            rate: 1.0,
            pitch: 0,
        }
    }
}
