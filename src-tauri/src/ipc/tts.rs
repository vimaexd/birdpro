use rodio::Decoder;
use std::io::Cursor;
use tauri::State;
use tokio::sync::Mutex as AsyncMutex;
use vrchat_osc::rosc::{OscMessage, OscPacket, OscType};

use crate::audio::AudioSetup;
use crate::backends::elevenlabs::ElevenlabsTTSProvider;
use crate::backends::msedge::MsEdgeTTSProvider;
#[cfg(windows)]
use crate::backends::windows::WindowsTTSProvider;
use crate::provider::{TTSBackend, TTSBackendError, TTSBackendInfo, TTSProvider, TTS_BACKENDS};
use crate::voice::Voice;
use crate::{get_platform, AppData};

#[tauri::command]
pub async fn tts_say(
    message: String,
    pitch: i32,
    rate: f64,
    provider: TTSBackend,
    voice: Voice,
    preview: Option<bool>,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), TTSBackendError> {
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
    let state = state.lock().await;

    // add pitch and rate to voice
    let mut voice_final = voice;
    voice_final.pitch = pitch;
    voice_final.rate = rate;

    let _bytes: Result<Vec<u8>, TTSBackendError> = match provider {
        TTSBackend::MsEdge => {
            MsEdgeTTSProvider::get_speech_bytes(message.as_str(), &voice_final, &state.config).await
        }
        TTSBackend::ElevenLabs => {
            ElevenlabsTTSProvider::get_speech_bytes(message.as_str(), &voice_final, &state.config)
                .await
        }
        #[cfg(windows)]
        TTSBackend::Windows => {
            WindowsTTSProvider::get_speech_bytes(message.as_str(), &voice_final, &state.config)
                .await
        }
    };

    let bytes = match _bytes {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let mut target_setups: Vec<&AudioSetup> = vec![];
    if preview.is_some() && preview.unwrap() == true {
        // currently hardcoded that setup index 1
        // is the preview device, so put to that
        if state.audio_setups.get(1).is_some() {
            target_setups.push(&state.audio_setups.get(1).unwrap().as_ref().unwrap());
        }
    } else {
        for setup in &state.audio_setups {
            if setup.is_none() {
                continue;
            }

            let setup_ref = setup.as_ref().unwrap();
            target_setups.push(setup_ref);
        }
    }

    // put out to all initialized audio setups
    for setup in target_setups {
        let src = Decoder::try_from(Cursor::new(bytes.clone()))
            .map_err(|_| TTSBackendError::DecodeError)?;

        setup.stream_handle.mixer().add(src);
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
    provider_id: TTSBackend,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<Vec<Voice>, TTSBackendError> {
    let state = state.lock().await;

    let _voices = match provider_id {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_voices(&state.config).await,
        TTSBackend::ElevenLabs => ElevenlabsTTSProvider::get_voices(&state.config).await,
        #[cfg(windows)]
        TTSBackend::Windows => WindowsTTSProvider::get_voices().await,
    };

    let voices = match _voices {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(voices)
}

#[tauri::command]
pub async fn tts_get_voice(state: State<'_, AsyncMutex<AppData>>) -> Result<Voice, ()> {
    let state = state.lock().await;
    Ok(state.voice.clone())
}

#[tauri::command]
pub async fn tts_set_voice(voice: Voice, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut state = state.lock().await;
    state.voice = voice.clone();

    log::info!("TTS voice changed to {}", voice.id);
    Ok(())
}

#[tauri::command]
pub async fn tts_get_providerlist() -> Result<Vec<TTSBackendInfo>, ()> {
    Ok(TTS_BACKENDS
        .iter()
        .cloned()
        .filter(|x| x.supported_platforms.contains(&get_platform()))
        .collect())
}

#[tauri::command]
pub async fn tts_get_provider(state: State<'_, AsyncMutex<AppData>>) -> Result<TTSBackend, ()> {
    let state = state.lock().await;
    Ok(state.provider)
}

#[tauri::command]
pub async fn tts_set_provider(
    provider: TTSBackend,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let mut state = state.lock().await;
    state.provider = provider.clone();

    let default_voice = match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_default_voice(),
        TTSBackend::ElevenLabs => ElevenlabsTTSProvider::get_default_voice(),
        #[cfg(windows)]
        TTSBackend::Windows => WindowsTTSProvider::get_default_voice(),
    };

    state.voice = default_voice;

    log::info!("TTS provider changed to {:?}", provider);

    Ok(())
}
