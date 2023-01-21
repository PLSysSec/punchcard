extern crate github_generators;
use github_generators::github_event;
use serde::{Deserialize, Serialize};

github_event!("CheckSuite", "Completed");
github_event!("Create");
github_event!("Delete");
github_event!("Deployment");
github_event!("DeploymentStatus");
github_event!("Discussion");
github_event!("DiscussionComment");
github_event!("Fork");
github_event!("Gollum");
github_event!("IssueComment");
github_event!("Issues");
github_event!("Label");
github_event!("MergeGroup");
github_event!("Milestone");
github_event!("PageBuild");
github_event!("Project");
github_event!("ProjectCard");
github_event!("ProjectColumn");
github_event!("Public");
github_event!("PullRequest");
github_event!("PullRequestReview");
github_event!("PullRequestReviewComment");
github_event!("PullRequestTarget");
github_event!("Push");
github_event!("RegistryPackage");
github_event!("Release");
github_event!("RepositoryDispatch");
github_event!("Schedule");
github_event!("Status");
github_event!("Watch");
github_event!("WorkflowCall");
github_event!("WorkflowDispatch");
github_event!("WorkflowRun");

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowEvents {
    pub branch_protection_rule: Option<BranchProtectionRule>,
    pub check_run: Option<CheckRun>,
    pub check_suite: Option<CheckSuite>,
    pub create: Option<Create>,
    pub delete: Option<Delete>,
    pub deployment: Option<Deployment>,
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
/*
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
*/