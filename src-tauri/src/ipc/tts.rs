use std::io::Cursor;
use rodio::{Decoder, DeviceTrait, cpal};
use tauri::State;
use tauri::{Manager};
use tokio::sync::Mutex as AsyncMutex;
use crate::backends::{msedge::{MsEdgeTTSProvider}};
use crate::provider::{TTS_BACKENDS, TTSBackend, TTSProvider, TTSBackendInfo};
use crate::AppData;

#[tauri::command]
pub async fn tts_say(message: String, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    if(message.is_empty()) {
        // empty message so skip
        // this should be handled on frontend, this is just a failsafe to prevent weird errors
        return Ok(())
    }

    println!("{}", message);
    let state = state.lock().await;

    let bytes: Vec<u8> = match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_speech_bytes(message.as_str(), &state.voice)
            .await.unwrap()
    };

    let source = Decoder::try_from(Cursor::new(bytes)).unwrap();
    state.audio_setup.stream_handle.mixer().add(source);

    Ok(())
}

#[tauri::command]
pub async fn tts_get_voicelist(state: State<'_, AsyncMutex<AppData>>) -> Result<Vec<String>, ()> {
    let state = state.lock().await;

    let voices = match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_voices()
    };

    Ok(voices)
}

#[tauri::command]
pub async fn tts_get_voice(state: State<'_, AsyncMutex<AppData>>) -> Result<String, ()> {
    let mut state = state.lock().await;
    Ok(state.voice.clone())
}

#[tauri::command]
pub async fn tts_set_voice(voice: String, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut state = state.lock().await;
    state.voice = voice.clone();

    println!("voice changed to {}", voice);
    Ok(())
}

#[tauri::command]
pub async fn tts_get_providerlist() -> Result<Vec<TTSBackendInfo>, ()> {
    Ok(TTS_BACKENDS.to_vec())
}

#[tauri::command]
pub async fn tts_get_provider(state: State<'_, AsyncMutex<AppData>>) -> Result<TTSBackend, ()> {
    let mut state = state.lock().await;
    Ok(state.provider)
}
