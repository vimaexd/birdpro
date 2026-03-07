use rodio::{Decoder, Sink, Source};
use std::io::Cursor;
use std::sync::OnceLock;
use tauri::State;
use tokio::sync::Mutex as AsyncMutex;
use vrchat_osc::rosc::{OscMessage, OscPacket, OscType};

use crate::audio::BirdSink;
use crate::backends::elevenlabs::ElevenlabsTTSProvider;
use crate::backends::msedge::MsEdgeTTSProvider;

use crate::backends::tiktok::TiktokTTSProvider;
#[cfg(windows)]
use crate::backends::windows::WindowsTTSProvider;
use crate::provider::{TTSProviderType, TTSProviderError, TTSProviderInfo, TTSProvider, TTS_PROVIDERS};
use crate::voice::{Voice, VoiceWithSettings};
use crate::{get_platform, AppData};

pub static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

#[tauri::command]
pub async fn tts_say(
    message: String,
    pitch: i32,
    rate: f64,
    provider: TTSProviderType,
    voice: Voice,
    preview: Option<bool>,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), TTSProviderError> {
    if message.is_empty() {
        // empty message so skip
        // this should be handled on frontend, this is just a failsafe to prevent weird errors
        return Ok(());
    }

    log::info!(
        "TTS speech: \"{}\", pitch: {}, rate: {}",
        message,
        pitch,
        rate
    );
    let mut state = state.lock().await;

    // add pitch and rate to voice
    let voice_final = VoiceWithSettings {
        voice: voice,
        pitch: pitch,
        rate: rate
    };

    let _bytes: Result<Vec<u8>, TTSProviderError> = match provider {
        TTSProviderType::MsEdge => {
            MsEdgeTTSProvider::get_speech_bytes(message.as_str(), &voice_final, &state.config).await
        }
        TTSProviderType::ElevenLabs => {
            ElevenlabsTTSProvider::get_speech_bytes(message.as_str(), &voice_final, &state.config)
                .await
        }
        TTSProviderType::Tiktok => {
            TiktokTTSProvider::get_speech_bytes(message.as_str(), &voice_final, &state.config)
                .await
        }
        #[cfg(windows)]
        TTSProviderType::Windows => {
            WindowsTTSProvider::get_speech_bytes(message.as_str(), &voice_final, &state.config)
                .await
        }
    };

    let bytes = match _bytes {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // clean up previous finished sinks
    state.audio_sinks.retain(|sink| !sink.sink.empty());

    let mut target_setup_ids: Vec<usize> = vec![];
    if preview.unwrap_or(false) {
        // currently hardcoded that setup index 1
        // is the preview device, so put to that
        if state.audio_setups.get(1).is_some() {
            target_setup_ids.push(1);
        }
    } else {
        for (i, setup) in state.audio_setups.iter().enumerate() {
            if setup.is_some() {
                target_setup_ids.push(i);
            }
        }
    }

    let speed = 1.0 + (pitch as f32 / 100.0);

    // put out to all initialized audio setups
    for setup in target_setup_ids {
        let audiosetup = state.audio_setups[setup].as_ref().unwrap();
        let src = Decoder::try_from(Cursor::new(bytes.clone()))
            .map_err(|_| TTSProviderError::DecodeError)?
            .speed(speed);

        let sink = Sink::connect_new(&audiosetup.stream_handle.mixer());
        sink.append(src);

        // get sink volume level
        let vols = state.config["volumes"].as_array().unwrap();
        sink.set_volume(vols[setup].as_f64().unwrap_or(1.0) as f32);

        state.audio_sinks.push(BirdSink {
            sink: sink,
            setup_index: setup,
        });
    }

    // if vrcosc is active, send the message there
    if state.vrc_osc.is_some() {
        let msg = OscMessage {
            addr: "/chatbox/input".to_string(),
            args: vec![
                OscType::String(message),
                OscType::Bool(true), // send immediately
            ],
        };

        state
            .vrc_osc
            .as_ref()
            .unwrap()
            .send(OscPacket::Message(msg), "VRChat-Client-*")
            .await
            .unwrap();
    }

    Ok(())
}

#[tauri::command]
pub async fn tts_get_voicelist(
    provider_id: TTSProviderType,
    state: State<'_, AsyncMutex<AppData>>,
    handle: tauri::AppHandle
) -> Result<Vec<Voice>, TTSProviderError> {
    let state = state.lock().await;

    // set app handle so below functions can access it
    // for resolving paths
    APP_HANDLE.set(handle).ok();

    let _voices = match provider_id {
        TTSProviderType::MsEdge => MsEdgeTTSProvider::get_voices(&state.config).await,
        TTSProviderType::ElevenLabs => ElevenlabsTTSProvider::get_voices(&state.config).await,
        TTSProviderType::Tiktok => TiktokTTSProvider::get_voices(&state.config).await,
        #[cfg(windows)]
        TTSProviderType::Windows => WindowsTTSProvider::get_voices(&state.config).await,
    };

    let voices = match _voices {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(voices)
}

#[tauri::command]
pub async fn tts_get_providerlist() -> Result<Vec<TTSProviderInfo>, ()> {
    Ok(TTS_PROVIDERS
        .iter()
        .cloned()
        .filter(|x| x.supported_platforms.contains(&get_platform()))
        .collect())
}

#[tauri::command]
pub async fn tts_get_default_provider(
    _state: State<'_, AsyncMutex<AppData>>,
) -> Result<TTSProviderInfo, ()> {
    Ok(*TTS_PROVIDERS.first().unwrap())
}

#[tauri::command]
pub async fn tts_get_default_voice(
    provider: TTSProviderType,
    _state: State<'_, AsyncMutex<AppData>>,
) -> Result<Voice, TTSProviderError> {
    let default_voice = match provider {
        TTSProviderType::MsEdge => MsEdgeTTSProvider::get_default_voice(),
        TTSProviderType::ElevenLabs => ElevenlabsTTSProvider::get_default_voice(),
        TTSProviderType::Tiktok => TiktokTTSProvider::get_default_voice(),
        #[cfg(windows)]
        TTSProviderType::Windows => WindowsTTSProvider::get_default_voice(),
    };

    Ok(default_voice)
}
