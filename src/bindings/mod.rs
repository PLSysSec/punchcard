use std::fs;
use std::path::Path;

pub use events::*;
pub use workflow::*;

pub fn read_yaml<T: AsRef<Path>>(file: T) -> String {
    fs::read_to_string(file).expect("File should exist")
}

mod events;
mod concurrency;
mod workflow;
