pub struct AudioSetup {
    pub stream_handle: rodio::stream::OutputStream,
    pub sink: rodio::Sink
}

impl AudioSetup {
    pub fn new() -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        println!("{}", "audio setup created");

        AudioSetup {
            stream_handle,
            sink
        }
    }
}
