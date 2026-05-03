use cpal::DeviceDescription;
use rodio::cpal::{self, traits::HostTrait};
use rodio::DeviceTrait;
use serde::{Deserialize, Serialize};

// sent over IPC to the frontend to display
// stats about audio from rodio
#[derive(Deserialize, Serialize)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub sample_rate: u32,
    pub bit_depth: usize,
}

#[derive(Deserialize, Serialize)]
pub enum AudioSetupError {
    DeviceNoLongerExists,
    NoDefaultDevice,
    DeviceOpenFailed,
    StreamOpenFailed,
}

pub struct AudioSetup {
    pub device: rodio::Device,
    pub stream_handle: rodio::stream::MixerDeviceSink,
    pub sink: rodio::Player,
}

impl AudioSetup {
    pub fn new() -> Result<Self, AudioSetupError> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(AudioSetupError::NoDefaultDevice);

        Self::from_device(device?)
    }

    pub fn from_device(device: rodio::Device) -> Result<Self, AudioSetupError> {
        let stream_handle = rodio::DeviceSinkBuilder::from_device(device.clone())
            .map_err(|_| AudioSetupError::DeviceOpenFailed)?
            .open_sink_or_fallback()
            .map_err(|_| AudioSetupError::StreamOpenFailed)?;

        let sink = rodio::Player::connect_new(&stream_handle.mixer());

        log::info!(
            "audio setup created with device \"{}\"",
            get_device_display_name(device.clone().description().unwrap())
        );

        Ok(Self {
            device,
            stream_handle,
            sink,
        })
    }
}

// used for display in the ui and matching -> cpal devices
// modifying this will break stored user device prefs!
pub fn get_device_display_name(device: DeviceDescription) -> String {
    let name = device.name().to_string();
    let driver = device.driver().unwrap_or("").to_string();
    format!("{name} ({driver})")
}

// helper to be able to differenciate different tts sinks to
// different audio outputs
pub struct BirdPlayer {
    pub sink: rodio::Player,
    pub setup_index: usize,
}
