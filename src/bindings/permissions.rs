use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Permissions {
    pub actions: Option<PermissionType>,
    pub checks: Option<PermissionType>,
    pub contents: Option<PermissionType>,
    pub deployments: Option<PermissionType>,
    pub id_token: Option<PermissionType>,
    pub issues: Option<PermissionType>,
    pub discussions: Option<PermissionType>,
    pub packages: Option<PermissionType>,
    pub pages: Option<PermissionType>,
    pub pull_requests: Option<PermissionType>,
    pub repository_projects: Option<PermissionType>,
    pub security_events: Option<PermissionType>,
    pub statuses: Option<PermissionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionType {
    Read,
    Write,
    None,
}
