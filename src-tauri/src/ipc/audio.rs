use std::io::{Cursor, Read};

use crate::audio::{AudioDeviceInfo, AudioSetup, BirdSink};
use crate::AppData;
use rodio::cpal::traits::HostTrait;
use rodio::{cpal, DeviceTrait, Source};
use tauri::path::BaseDirectory;
use tauri::{Manager, State};
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

    log::info!("{:#?}", volume);

    let target_setup = state.audio_setups.get(setup_idx);
    if target_setup.is_none() {
        return Ok(());
    }

    // set current sinks to that volume
    for sink in &state.audio_sinks {
        if sink.setup_index == setup_idx {
            sink.sink.set_volume(volume);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn audio_stop_all(
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let st = state.lock().await;

    for sink in &st.audio_sinks {
        sink.sink.stop();
    }
    Ok(())
}

#[tauri::command]
pub async fn audio_typingindicator_start(
    state: State<'_, AsyncMutex<AppData>>,
    handle: tauri::AppHandle
) -> Result<(), ()> {
    let mut st = state.lock().await;

    // load sound
    let file_path = handle.path().resolve("assets/snd_talking.wav", BaseDirectory::Resource).unwrap();
    let audio_data = std::fs::read(&file_path).unwrap();

    for i in 0..st.audio_setups.len() {
        let setup = &st.audio_setups[i];
        if setup.is_some() {
            let sink = rodio::Sink::connect_new(&setup.as_ref().unwrap().stream_handle.mixer());
            let src = rodio::Decoder::try_from(Cursor::new(audio_data.clone())).unwrap();
            sink.set_volume(0.8);
            sink.append(src.repeat_infinite());
            sink.play();

            let bs = BirdSink {
                setup_index: i,
                sink: sink
            };

            st.audio_sinks_typingindicator.push(bs);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn audio_typingindicator_stop(
    state: State<'_, AsyncMutex<AppData>>,
) -> Result<(), ()> {
    let mut st = state.lock().await;

    for ti in st.audio_sinks_typingindicator.drain(..) {
        tokio::spawn(async move {
            let fade_duration = std::time::Duration::from_millis(300);
            let steps = 20;
            let step_duration = fade_duration / steps;
            let current_vol = ti.sink.volume();

            for i in (0..steps).rev() {
                let new_vol = current_vol * (i as f32 / steps as f32);
                ti.sink.set_volume(new_vol);
                tokio::time::sleep(step_duration).await;
            }

            ti.sink.stop();
        });
    }
    st.audio_sinks_typingindicator = vec![];
    Ok(())
}
