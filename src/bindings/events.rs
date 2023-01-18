use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowEvents {
    pub branch_protection_rule: Option<BranchProtectionRule>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BranchProtectionRule {
    pub types: Option<Vec<BranchProtectionActivityTypes>>,
    pub github_sha: Option<String>,
    pub github_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionActivityTypes {
    Created,
    Edited,
    Deleted,
}
