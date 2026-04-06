pub mod audio;
pub mod backends;
pub mod provider;
pub mod voice;
pub mod hrm;

#[macro_use]
pub mod ipc;

use crate::audio::{AudioSetup, BirdPlayer};
use crate::hrm::PulsoidService;
use crate::provider::TTSProviderPlatform;
use log::*;
use serde_json::Value;
use std::sync::Arc;
use tauri::window::Color;
use tauri::{Manager};
use tokio::sync::Mutex as AsyncMutex;
use vrchat_osc::VRChatOSC;

pub struct AppData {
    config: Value,
    audio_setups: Vec<Option<AudioSetup>>,
    audio_sinks: Vec<BirdPlayer>,
    audio_sinks_typingindicator: Vec<BirdPlayer>,
    vrc_osc: Option<Arc<VRChatOSC>>,
    pulsoid_service: Option<PulsoidService>
}

pub fn get_platform() -> TTSProviderPlatform {
    match std::env::consts::OS {
        "windows" => TTSProviderPlatform::Windows,
        "linux" => TTSProviderPlatform::Linux,
        "macos" => TTSProviderPlatform::MacOS,
        _ => TTSProviderPlatform::Unknown,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{} {} {}] {}",
                        humantime::format_rfc3339(std::time::SystemTime::now()),
                        record.level(),
                        // assuming if we dont have a module path then it's
                        // probably from the frontend
                        record.module_path().unwrap_or("frontend"),
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
                .max_file_size(10_000_000)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            info!("Bird Pro v{}", app.package_info().version);

            // set background color to avoid flashes on startup
            let win = app.get_webview_window("main").unwrap();
            win.set_background_color(Some(Color::from([17, 18, 16])))
                .unwrap();

            // create default audio device output
            let main_output = AudioSetup::new().ok();

            app.manage(AsyncMutex::new(AppData {
                config: Value::from(0), // will be updated by frontend
                audio_setups: vec![main_output, None, None, None],
                audio_sinks: vec![],
                audio_sinks_typingindicator: vec![],
                vrc_osc: None,
                pulsoid_service: None
            }));

            info!("Setup complete, waiting for webview");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::ipc::tts::tts_say,
            crate::ipc::tts::tts_get_voicelist,
            crate::ipc::tts::tts_get_providerlist,
            crate::ipc::tts::tts_get_default_provider,
            crate::ipc::tts::tts_get_default_voice,
            crate::ipc::audio::audio_get_devices,
            crate::ipc::audio::audio_get_device,
            crate::ipc::audio::audio_set_device,
            crate::ipc::audio::audio_destroy,
            crate::ipc::audio::audio_get_volume,
            crate::ipc::audio::audio_set_volume,
            crate::ipc::audio::audio_stop_all,
            crate::ipc::audio::audio_typingindicator_start,
            crate::ipc::audio::audio_typingindicator_stop,
            crate::ipc::osc::osc_start,
            crate::ipc::osc::osc_stop,
            crate::ipc::osc::osc_typing_indicator,
            crate::ipc::osc::hrm_svc_start,
            crate::ipc::osc::hrm_svc_stop,
            crate::ipc::error::get_error_text,
            crate::ipc::config::update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
