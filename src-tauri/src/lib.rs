mod backends;
mod audio;

use std::io::Cursor;
use std::sync::Mutex;
use rodio::cpal::traits::HostTrait;
use rodio::{Decoder, Device, DeviceTrait, cpal};
use tauri::State;
use tauri::{Builder, Manager};
use crate::audio::AudioSetup;
use crate::backends::{msedge::{self, MsEdgeTTSProvider}, tts::{TTSBackend, TTSProvider}};

struct AppData {
  provider: TTSBackend,
  audio_setup: AudioSetup,
  voice: String
}

#[tauri::command]
fn tts_say(message: &str, state: State<'_, Mutex<AppData>>) {
    println!("{}", message);
    let mut state = state.lock().unwrap();

    let bytes: Vec<u8> = match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_speech_bytes(message, &state.voice)
    };

    let source = Decoder::try_from(Cursor::new(bytes)).unwrap();
    let snk = state.audio_setup.stream_handle.mixer().add(source);
}

#[tauri::command]
fn tts_get_voices(state: State<'_, Mutex<AppData>>) -> Vec<String> {
    let mut state = state.lock().unwrap();

    match state.provider {
        TTSBackend::MsEdge => MsEdgeTTSProvider::get_voices()
    }
}

#[tauri::command]
fn tts_set_voice(voice: String, state: State<'_, Mutex<AppData>>) {
    let mut state = state.lock().unwrap();
    state.voice = voice.clone();

    println!("voice changed to {}", voice);
}

#[tauri::command]
fn audio_get_devices() -> Vec<String> {
    let devices = cpal::default_host().output_devices().unwrap();

    let device_list: Vec<String> = devices.map(|x| {
        x.name().unwrap()
    }).collect();

    device_list
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppData {
                provider: TTSBackend::MsEdge,
                audio_setup: AudioSetup::new(),
                voice: MsEdgeTTSProvider::get_default_voice()
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![tts_say, tts_get_voices, tts_set_voice, audio_get_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
