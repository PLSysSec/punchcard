use std::fs;

pub fn read_dhall(file: String) -> String {
    fs::read_to_string(file).expect("File should exist")
}
