use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Matrix {
    pub matrix: IndexMap<String, Vec<String>>,
    #[serde(rename = "fail-fast")]
    pub fail_fast: Option<bool>,
    #[serde(rename = "max-parallel")]
    pub max_parallel: Option<u64>,
}
