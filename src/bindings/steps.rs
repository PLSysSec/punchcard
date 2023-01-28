use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

// TODO: Either Run or Uses is required. I should use an
//       enum to ensure this statically.

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kabob-case")]
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
    pub continue_on_error: Option<bool>,
    pub timeout_minutes: Option<String>,
    pub strategy: Option<Matrix>,
}
