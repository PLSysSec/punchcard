use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Concurrency {
    Group: String,
    #[serde(rename = "cancel-in-progress")]
    CancelInProgress: Option<bool>,
}
