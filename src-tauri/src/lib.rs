pub mod provider;
pub mod audio;
pub mod backends;
pub mod voice;

#[macro_use]
pub mod ipc;

use log::*;
use fern::colors::{Color, ColoredLevelConfig};
use tauri::{Manager};
use tokio::sync::Mutex as AsyncMutex;
use crate::audio::AudioSetup;
use crate::backends::{msedge::{MsEdgeTTSProvider}};
use crate::provider::{TTSBackend, TTSProvider, TTSProviderPlatform};
use crate::voice::Voice;

pub struct AppData {
  provider: TTSBackend,
  audio_setups: Vec<AudioSetup>,
  voice: Voice,
}

pub fn get_platform() -> TTSProviderPlatform {
    match std::env::consts::OS {
        "windows" => TTSProviderPlatform::Windows,
        "linux" => TTSProviderPlatform::Linux,
        _ => TTSProviderPlatform::Unknown
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                colors_line.color(record.level()),
                message
            ))
        })
        // .filter(|m| m.target() == "birdcore")
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        // Apply globally
        .apply().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            info!("Bird Pro v{}", app.package_info().version);

            let audio = AudioSetup::new();
            app.manage(AsyncMutex::new(AppData {
                provider: TTSBackend::MsEdge,
                audio_setups: vec![audio],
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
            crate::ipc::tts::tts_set_provider,
            crate::ipc::audio::audio_get_devices,
            crate::ipc::audio::audio_get_device,
            crate::ipc::audio::audio_set_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
