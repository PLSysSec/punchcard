use crate::Config;
use std::fs::File;
use std::io::Write;

/// `initialize` creates a .punchcard.yml file if one does not already exist.
/// TODO: Check for a .git repo (the repo root). Continue looking upward
/// until you find one.
pub fn initialize() {
    let conf = Config::default();
    let mut file = File::create(".punchcard.yml").unwrap();
    let yaml = serde_yaml::to_string(&conf).expect("Marshal to string");
    file.write_all(yaml.as_bytes()).expect("Write to file");
}
