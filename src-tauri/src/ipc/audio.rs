use rodio::cpal::traits::HostTrait;
use rodio::{DeviceTrait, cpal};
use tauri::State;
use tokio::sync::Mutex as AsyncMutex;
use crate::audio::AudioSetup;
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
pub async fn audio_get_device(setup_idx: usize, state: State<'_, AsyncMutex<AppData>>) -> Result<Option<String>, ()> {
    let state = state.lock().await;
    if state.audio_setups.get(setup_idx).is_none() {
        return Ok(None)
    }
    Ok(Some(state.audio_setups[setup_idx].device.name().unwrap()))
}

#[tauri::command]
pub async fn audio_set_device(setup_idx: usize, device_name: String, state: State<'_, AsyncMutex<AppData>>) -> Result<(), ()> {
    let mut state = state.lock().await;

    //find the device based on the name
    let mut devices = cpal::default_host().output_devices().unwrap();
    let device = devices.find(|x| {
        x.name().unwrap() == device_name
    }).expect("failed to resolve device");

    log::info!("[IPC] Audio setup update (setup {})", setup_idx);
    let setup = AudioSetup::from_device(device);
    state.audio_setups[setup_idx] = setup;


    Ok(())
}
