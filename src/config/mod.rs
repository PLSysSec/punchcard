use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    disable_all: bool,
    enable: Vec<String>,
}
