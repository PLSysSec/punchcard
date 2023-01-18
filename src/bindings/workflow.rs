use serde::{Serialize, Deserialize};

use super::WorkflowEvents;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Workflow {
    pub name: Option<String>,
    pub run_name: Option<String>,
    pub on: WorkflowEvents,
}
