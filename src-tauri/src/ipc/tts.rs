use std::io::Cursor;
use rodio::{Decoder};
use tauri::State;
use tokio::sync::Mutex as AsyncMutex;

#[cfg(windows)]
use crate::backends::windows::WindowsTTSProvider;
// use crate::backends::tiktok::TiktokTTSProvider;
use crate::backends::msedge::MsEdgeTTSProvider;
use crate::provider::{TTS_BACKENDS, TTSBackend, TTSProvider, TTSBackendInfo};
use crate::voice::{Voice};
use crate::{AppData, get_platform};

#[tauri::command]
pub async fn tts_say(message: String, pitch: i32, rate: f64, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    if message.is_empty() {
        // empty message so skip
        // this should be handled on frontend, this is just a failsafe to prevent weird errors
        return Ok(())
    }

    log::info!("[IPC] TTS speech: \"{}\", pitch: {}, rate: {}", message, pitch, rate);
    let state = state.lock().await;

    // add pitch and rate to voice
    let mut voice_final = state.voice.clone();
    voice_final.pitch = pitch;
    voice_final.rate = rate;

    let bytes: Result<Vec<u8>, ()> = match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_speech_bytes(message.as_str(), &voice_final)
            .await,
        // TTSBackend::TikTok => TiktokTTSProvider::get_speech_bytes(message.as_str(), &voice_final)
        //     .await,

        #[cfg(windows)]
        TTSBackend::Windows => WindowsTTSProvider::get_speech_bytes(message.as_str(), &voice_final)
            .await
    };

    if bytes.is_err() {
        // TODO: display error to user
        return Ok(()) // drop the promise to make the
    }


    for setup in &state.audio_setups {
        let src = Decoder::try_from(Cursor::new(bytes.clone()?)).unwrap();
        setup.stream_handle.mixer().add(src);
    }

    Ok(())
}

#[tauri::command]
pub async fn tts_get_voicelist(state: State<'_, AsyncMutex<AppData>>) -> Result<Vec<Voice>, ()> {
    let state = state.lock().await;

    let voices = match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_voices(),
        // TTSBackend::TikTok => TiktokTTSProvider::get_voices(),

        #[cfg(windows)]
        TTSBackend::Windows => WindowsTTSProvider::get_voices()
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

    log::info!("[IPC] TTS voice changed to {}", voice.id);
    Ok(())
}

#[tauri::command]
pub async fn tts_get_providerlist() -> Result<Vec<TTSBackendInfo>, ()> {
    Ok(
        TTS_BACKENDS.iter().cloned()
            .filter(|x|
                x.supported_platforms.contains(&get_platform())
            )
            .collect()
    )
}

#[tauri::command]
pub async fn tts_get_provider(state: State<'_, AsyncMutex<AppData>>) -> Result<TTSBackend, ()> {
    let state = state.lock().await;
    Ok(state.provider)
}

#[tauri::command]
pub async fn tts_set_provider(provider: TTSBackend, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut state = state.lock().await;
    state.provider = provider.clone();

    let default_voice = match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_default_voice(),
        // TTSBackend::TikTok => TiktokTTSProvider::get_default_voice(),

        #[cfg(windows)]
        TTSBackend::Windows => WindowsTTSProvider::get_default_voice()
    };

    state.voice = default_voice;

    log::info!("[IPC] TTS provider changed to {:?}", provider);

    Ok(())
}
