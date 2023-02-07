use crate::bindings::matrix::Matrix;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::bindings::BoolOrExpr;

// TODO: Either Run or Uses is required. I should use an
//       enum to ensure this statically.

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub id: Option<String>,
    #[serde(rename = "if")]
    pub if_expr: Option<String>,
    pub name: Option<String>,
    pub uses: Option<String>,
    pub run: Option<String>,
    pub shell: Option<String>,
    pub with: Option<IndexMap<String, String>>,
    pub env: Option<IndexMap<String, String>>,
    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<BoolOrExpr>,
    #[serde(rename = "timeout-minutes")]
    pub timeout_minutes: Option<String>,
    pub strategy: Option<Matrix>,
}
