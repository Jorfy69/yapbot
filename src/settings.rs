use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
 pub struct Keycombos {
    pub name: String,
    pub keys: Vec<String>,
    pub audio: String,
}
#[derive(Serialize, Deserialize, Debug)]
 pub struct Settings { 
    logging_enabled: bool, 
    logging_delay: u64, 
    logging_file: String, 
    audio_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub key_combos: Vec<Keycombos>,
    pub exit_key: String,
    pub settings: Settings,
}


pub fn read_settings(filename: &str) -> Config {
    let mut file = File::open(filename).unwrap();
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).unwrap();

    let json = serde_json::from_str(&json_data).expect("unable to parse json");
    return json;
}

