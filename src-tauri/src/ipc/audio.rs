use crate::audio::{AudioDeviceInfo, AudioSetup};
use crate::AppData;
use rodio::cpal::traits::HostTrait;
use rodio::{cpal, DeviceTrait};
use tauri::State;
use tokio::sync::Mutex as AsyncMutex;

#[tauri::command]
pub fn audio_get_devices() -> Vec<String> {
    let devices = cpal::default_host().output_devices().unwrap();

    let device_list: Vec<String> = devices.map(|x| x.name().unwrap()).collect();

    device_list
}

#[tauri::command]
pub async fn audio_get_device(
    setup_idx: usize,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<Option<AudioDeviceInfo>, ()> {
    let state = state.lock().await;
    if state.audio_setups.get(setup_idx).is_none() {
        return Ok(None);
    }
    let setup = state.audio_setups[setup_idx]
        .as_ref()
        .expect("failed to get audio setup");

    Ok(Some(AudioDeviceInfo {
        name: setup.device.name().unwrap(),
        sample_rate: setup.stream_handle.config().sample_rate(),
        bit_depth: setup.stream_handle.config().sample_format().sample_size(),
    }))
}

#[tauri::command]
pub async fn audio_set_device(
    setup_idx: usize,
    device_name: String,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), String> {
    let mut state = state.lock().await;

    //find the device based on the name
    let mut devices = cpal::default_host().output_devices().unwrap();
    let device = match devices.find(|x| x.name().unwrap() == device_name) {
        Some(d) => d,
        None => {
            return Err(format!("Device '{}' isn't available", device_name));
        }
    };

    log::info!("Audio setup update (setup {})", setup_idx);
    let setup = AudioSetup::from_device(device);
    state.audio_setups[setup_idx] = Some(setup);

    Ok(())
}

#[tauri::command]
pub async fn audio_destroy(
    setup_idx: usize,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let mut state = state.lock().await;

    let target_setup = state.audio_setups.get(setup_idx);
    if target_setup.is_none() {
        return Ok(());
    }

    log::info!("destroying audio setup {}", setup_idx);

    // stop output
    state.audio_setups[setup_idx] = None;

    Ok(())
}

#[tauri::command]
pub async fn audio_get_volume(
    setup_idx: usize,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<f32, ()> {
    let state = state.lock().await;

    let target_setup = state.audio_setups.get(setup_idx);
    if target_setup.is_none() {
        return Ok(0.0);
    }

    let vol = target_setup.unwrap().as_ref().unwrap().sink.volume();
    Ok(vol)
}

#[tauri::command]
pub async fn audio_set_volume(
    setup_idx: usize,
    volume: f32,
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let state = state.lock().await;

    let target_setup = state.audio_setups.get(setup_idx);
    if target_setup.is_none() {
        return Ok(());
    }

    target_setup
        .unwrap()
        .as_ref()
        .unwrap()
        .sink
        .set_volume(volume);
    Ok(())
}

#[tauri::command]
pub async fn audio_stop_all(
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let st = state.lock().await;

    for setup in &st.audio_setups {
        if setup.is_none() {
            continue;
        }
        setup.as_ref().unwrap().sink.stop();
    }
    Ok(())
}
