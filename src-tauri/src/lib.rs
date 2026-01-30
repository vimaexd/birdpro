pub mod provider;
pub mod audio;
pub mod backends;

#[macro_use]
pub mod ipc;

use std::io::Cursor;
use rodio::cpal::traits::HostTrait;
use rodio::{Decoder, DeviceTrait, cpal};
use tauri::State;
use tauri::{Manager};
use tokio::sync::Mutex as AsyncMutex;
use std::sync::Mutex as SyncMutex;
use crate::audio::AudioSetup;
use crate::backends::{msedge::{MsEdgeTTSProvider}};
use crate::provider::{TTSBackend, TTSProvider};

pub struct AppData {
  provider: TTSBackend,
  audio_setup: AudioSetup,
  voice: String
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AsyncMutex::new(AppData {
                provider: TTSBackend::MsEdge,
                audio_setup: AudioSetup::new(),
                voice: MsEdgeTTSProvider::get_default_voice()
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::ipc::tts::tts_say,
            crate::ipc::tts::tts_get_voicelist,
            crate::ipc::tts::tts_set_voice,
            crate::ipc::tts::tts_get_voice,
            crate::ipc::tts::tts_get_providerlist,
            crate::ipc::tts::tts_get_provider,
            crate::ipc::audio::audio_get_devices,
            crate::ipc::audio::audio_get_device,
            crate::ipc::audio::audio_set_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
