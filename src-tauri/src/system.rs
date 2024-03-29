use std::{fs::read_to_string, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    generation_path: String,
}

impl Config {
    fn new(file_path: &Path) -> Config {
        let raw_str = read_to_string(file_path).expect("Failed on loading config.");
        toml::from_str(&raw_str).expect("Fail on parsing config.")
    }
    fn print(&self) {
        println!("{}", self.generation_path)
    }
}
