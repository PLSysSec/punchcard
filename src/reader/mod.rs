use std::fs;

/// `load_spec` will read a file from disk and convert it from
/// dhall into a Workflow specification.
pub fn load_spec() -> Spec {}

pub fn read_dhall(file: String) -> String {
    fs::read_to_string(file).expect("File should exist")
}
