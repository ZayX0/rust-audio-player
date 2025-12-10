use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct AudioEngine {
    stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let stream = match OutputStreamBuilder::open_default_stream() {
            Ok(handler) => handler,
            Err(e) => return Err(Box::new(e)),
        };
        let (sink, _queue) = Sink::new();
        Ok(Self {
            stream,
            sink: Arc::new(Mutex::new(sink)),
        })
    }

    pub fn load_and_play_file(&mut self, path: &std::path::Path) -> Result<(), Box<dyn Error>> {
        // stop previous sound
        let mut sink_guard = match self.sink.lock() {
            Ok(guard) => guard,
            Err(_e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to lock audio sink",
                )));
            }
        };

        // create a new sink for each track
        let new_sink = Sink::connect_new(&self.stream.mixer());
        *sink_guard = new_sink;

        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        sink_guard.append(source);
        sink_guard.play();
        Ok(())
    }

    pub fn pause(&self) {
        self.sink.lock().unwrap().pause();
    }

    pub fn play(&self) {
        self.sink.lock().unwrap().play();
    }

    pub fn set_volume(&self, vol: f32) {
        self.sink.lock().unwrap().set_volume(vol);
    }
}
