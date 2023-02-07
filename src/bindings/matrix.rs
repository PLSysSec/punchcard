use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::bindings::BoolOrExpr;
use crate::bindings::OneOrMany;

#[derive(Serialize, Deserialize, Debug)]
pub struct Matrix {
    pub matrix: IndexMap<String, OneOrMany<String, String>>,
    #[serde(rename = "fail-fast")]
    pub fail_fast: Option<BoolOrExpr>,
    #[serde(rename = "max-parallel")]
    pub max_parallel: Option<u64>,
}
