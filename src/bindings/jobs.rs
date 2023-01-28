use crate::bindings::concurrency::Concurrency;
use crate::bindings::defaults::Defaults;
use crate::bindings::environment::Environment;
use crate::bindings::matrix::Matrix;
use crate::bindings::one_or::OneOrMany;
use crate::bindings::permissions::Permissions;
use crate::bindings::steps::Step;
use crate::bindings::OneOrDictionary;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Job {
    pub name: Option<String>,
    pub permissions: Option<Permissions>,
    pub needs: Option<OneOrMany<String, String>>,
    #[serde(rename = "if")]
    pub if_expr: Option<String>,
    pub runs_on: OneOrMany<String, String>,
    pub environment: Option<OneOrDictionary<String, Environment>>,
    pub env: Option<IndexMap<String, String>>,
    pub concurrency: Option<OneOrDictionary<String, Concurrency>>,
    pub outputs: Option<IndexMap<String, String>>,
    pub defaults: Option<Defaults>,
    pub steps: Vec<Step>,
    pub continue_on_error: Option<bool>,
    pub timeout_minutes: Option<String>,
    pub strategy: Option<Matrix>,
    // TODO: Container.
    // TODO: Services.
    pub uses: Option<String>,
    pub with: Option<IndexMap<String, String>>,
    pub secrets: Option<IndexMap<String, String>>,
}
