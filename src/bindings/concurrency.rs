use serde::{Deserialize, Serialize};
use crate::bindings::BoolOrExpr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Concurrency {
    group: String,
    #[serde(rename = "cancel-in-progress")]
    cancel_in_progress: Option<BoolOrExpr>,
}
