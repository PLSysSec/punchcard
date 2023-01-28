use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kabob-case")]
pub struct Matrix {
    pub matrix: IndexMap<String, Vec<String>>,
    pub fail_fast: Option<bool>,
    pub max_parallel: Option<u64>,
}
