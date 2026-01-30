use rodio::cpal::{self, traits::HostTrait};
use rodio::DeviceTrait;

pub struct AudioSetup {
    pub device: rodio::Device,
    pub stream_handle: rodio::stream::OutputStream,
    pub sink: rodio::Sink
}

impl AudioSetup {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();

        Self::from_device(device)
    }

    pub fn from_device(device: rodio::Device) -> Self {
        println!("output: {}", device.clone().name().unwrap());

        let stream_handle = rodio::OutputStreamBuilder::from_device(device.clone())
            .expect("failed to open device")
            .open_stream_or_fallback()
            .expect("failed to open stream handle");


        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        println!("{}", "audio setup created");

        Self {
            device,
            stream_handle,
            sink
        }
    }
}
