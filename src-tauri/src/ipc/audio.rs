use rodio::cpal::traits::HostTrait;
use rodio::{Decoder, DeviceTrait, cpal};
use tauri::State;
use tokio::sync::Mutex as AsyncMutex;
use crate::audio::AudioSetup;
use crate::backends::{msedge::{MsEdgeTTSProvider}};
use crate::provider::{TTSBackend, TTSProvider};
use crate::AppData;

#[tauri::command]
pub fn audio_get_devices() -> Vec<String> {
    let devices = cpal::default_host().output_devices().unwrap();

    let device_list: Vec<String> = devices.map(|x| {
        x.name().unwrap()
    }).collect();

    device_list
}

#[tauri::command]
pub async fn audio_get_device(state: State<'_, AsyncMutex<AppData>>) -> Result<String, ()> {
    let state = state.lock().await;
    Ok(state.audio_setup.device.name().unwrap())
}

#[tauri::command]
pub async fn audio_set_device(device_name: String, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut state = state.lock().await;

    //find the device based on the name
    let mut devices = cpal::default_host().output_devices().unwrap();
    let device = devices.find(|x| {
        x.name().unwrap() == device_name
    }).expect("failed to resolve device");

    let setup = AudioSetup::from_device(device);

    state.audio_setup = setup;

    Ok(())
}
