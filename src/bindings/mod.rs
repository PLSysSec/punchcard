use std::fs;
use std::path::Path;

pub use events::*;
pub use jobs::*;
pub use one_or::*;
pub use permissions::*;
pub use steps::*;
pub use workflow::*;

pub fn read_yaml<T: AsRef<Path>>(file: T) -> String {
    fs::read_to_string(file).expect("File should exist")
}

mod concurrency;
mod defaults;
mod environment;
mod events;
mod jobs;
mod matrix;
mod one_or;
mod permissions;
mod steps;
mod workflow;
