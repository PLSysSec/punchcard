use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    disable_all: bool,
    enable: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            disable_all: false,
            enable: Vec::default(),
        }
    }
}
