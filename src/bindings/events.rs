use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowEvents {
    pub branch_protection_rule: Option<BranchProtectionRule>,
    pub check_run: Option<CheckRun>,
    pub check_suite: Option<CheckSuite>,
    pub create: Option<Create>,
    pub delete: Option<Delete>,
}


// Event: branch_protection_rule

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct BranchProtectionRule {
    pub types: Option<Vec<BranchProtectionActivityType>>,
    pub github_sha: Option<String>,
    pub github_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionActivityType {
    Created,
    Edited,
    Deleted,
}

// Event: check_run

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CheckRun {
    pub types: Option<Vec<CheckRunActivityType>>,
    pub github_sha: Option<String>,
    pub github_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunActivityType {
    Created,
    Rerequested,
    Completed,
    RequestedAction,
}

// Event: check_suite

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CheckSuite {
    pub types: Option<Vec<CheckSuiteActivityType>>,
    pub github_sha: Option<String>,
    pub github_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteActivityType {
    Completed,
}

// Event: create

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Create {
    pub github_sha: Option<String>,
    pub github_ref: Option<String>,
}

// Event: delete

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Delete {
    pub github_sha: Option<String>,
    pub github_ref: Option<String>,
}
