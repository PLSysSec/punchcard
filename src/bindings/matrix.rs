use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Matrix {
    pub matrix: IndexMap<String, Vec<String>>,
}
