#![allow(unused_imports)]
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};


pub fn play(name:   &str) {

  let (_stream, stream_handle) = OutputStream::try_default().unwrap();

  let file = BufReader::new(File::open(name).unwrap());

  let source = Decoder::new(file).unwrap();

  stream_handle.play_raw(source.convert_samples()).unwrap();

  std::thread::sleep(std::time::Duration::from_secs(5));

}