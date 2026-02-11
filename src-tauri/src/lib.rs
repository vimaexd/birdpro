pub mod audio;
pub mod backends;
pub mod provider;
pub mod voice;

#[macro_use]
pub mod ipc;

use crate::audio::AudioSetup;
use crate::backends::msedge::MsEdgeTTSProvider;
use crate::provider::{TTSBackend, TTSProvider, TTSProviderPlatform};
use crate::voice::Voice;
use std::sync::Arc;
use log::*;
use serde_json::Value;
use tauri::Manager;
use tauri::window::Color;
use tokio::sync::Mutex as AsyncMutex;
use vrchat_osc::VRChatOSC;

pub struct AppData {
    config: Value,
    provider: TTSBackend,
    audio_setups: Vec<Option<AudioSetup>>,
    voice: Voice,
    vrc_osc: Option<Arc<VRChatOSC>>
}

pub fn get_platform() -> TTSProviderPlatform {
    match std::env::consts::OS {
        "windows" => TTSProviderPlatform::Windows,
        "linux" => TTSProviderPlatform::Linux,
        _ => TTSProviderPlatform::Unknown,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{} {} {}] {}",
                        humantime::format_rfc3339(std::time::SystemTime::now()),
                        record.level(),
                        record.module_path().unwrap_or("unk"),
                        message
                    ))
                })
                .clear_targets()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            info!("Bird Pro v{}", app.package_info().version);

            // set background color to avoid flashes on startup
            let win = app.get_webview_window("main").unwrap();
            win.set_background_color(Some(Color::from([17, 18, 16]))).unwrap();

            let audio = AudioSetup::new();
            app.manage(AsyncMutex::new(AppData {
                config: Value::from(0), // will be updated by frontend
                provider: TTSBackend::MsEdge,
                audio_setups: vec![Some(audio), None, None, None],
                voice: MsEdgeTTSProvider::get_default_voice(),
                vrc_osc: None
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
            crate::ipc::tts::tts_set_provider,
            crate::ipc::audio::audio_get_devices,
            crate::ipc::audio::audio_get_device,
            crate::ipc::audio::audio_set_device,
            crate::ipc::audio::audio_destroy,
            crate::ipc::audio::audio_get_volume,
            crate::ipc::audio::audio_set_volume,
            crate::ipc::osc::osc_start,
            crate::ipc::osc::osc_stop,
            crate::ipc::osc::osc_typing_indicator,
            crate::ipc::error::get_error_text,
            crate::ipc::config::update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
