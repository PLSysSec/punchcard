use std::fs;
use std::path::PathBuf;

pub use events::{*};
pub use workflow::{*};

pub fn read_yaml(file: PathBuf) -> String {
    fs::read_to_string(file).expect("File should exist")    
}

mod events;
mod workflow;
