use std::fs::File;
use std::io::{BufRead,BufReader, Write};
use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, source::Source};

pub async fn main(){
}

pub fn play(path: &str, stream_handle: &OutputStreamHandle){
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_millis(400));
}
