extern crate github_generators;
use github_generators::github_event;
use serde::{Deserialize, Serialize};

github_event!("BranchProtectionRule", "Created", "Edited", "Deleted");
github_event!(
    "CheckRun",
    "Created",
    "Rerequested",
    "Completed",
    "RequestedAction"
);
github_event!("CheckSuite", "Completed");
github_event!("Create");
github_event!("Delete");
github_event!("Deployment");
github_event!("DeploymentStatus");
github_event!(
    "Discussion",
    "Created",
    "Edited",
    "Deleted",
    "Transferred",
    "Pinned",
    "Unpinned",
    "Labeled",
    "Unlabeled",
    "Locked",
    "Unlocked",
    "CategoryChanged",
    "Answered",
    "Unanswered"
);
github_event!("DiscussionComment", "Created", "Edited", "Deleted");
github_event!("Fork");
github_event!("Gollum");
github_event!("IssueComment", "Created", "Edited", "Deleted");
github_event!(
    "Issues",
    "Opened",
    "Edited",
    "Deleted",
    "Transferred",
    "Pinned",
    "Unpinned",
    "Closed",
    "Reopened",
    "Assigned",
    "unassigned",
    "Labeled",
    "Unlabeled",
    "Locked",
    "Unlocked",
    "Milestoned",
    "Demilestoned"
);
github_event!("Label", "Created", "Edited", "Deleted");
github_event!("MergeGroup", "ChecksRequested");
github_event!(
    "Milestone",
    "Created",
    "Closed",
    "Opened",
    "Edited",
    "Deleted"
);
github_event!("PageBuild");
github_event!("Project", "Created", "Closed", "Reopened", "Edited", "Deleted");
github_event!(
    "ProjectCard",
    "Created",
    "Moved",
    "Converted",
    "Edited",
    "Deleted"
);
github_event!("ProjectColumn", "Created", "Updated", "Moved", "Deleted");
github_event!("Public");
github_event!(
    "PullRequest",
    "Assigned",
    "Unassigned",
    "Labeled",
    "Unlabeled",
    "Opened",
    "Edited",
    "Closed",
    "Reopened",
    "Synchronized",
    "ConvertedToDraft",
    "ReadyForReview",
    "Locked",
    "Unlocked",
    "ReviewRequested",
    "ReviewRequestedRemoved",
    "AutoMergeEnabled",
    "AutoMergeDisabled",
);
github_event!("PullRequestReview", "Submitted", "Edited", "Dismissed");
github_event!("PullRequestReviewComment", "Created", "Edited", "Deleted");
github_event!(
    "PullRequestTarget",
    "Assigned",
    "Unassigned",
    "Labeled",
    "Unlabeled",
    "Opened",
    "Edited",
    "Closed",
    "Reopened",
    "Synchronized",
    "ConvertedToDraft",
    "ReadyForReview",
    "Locked",
    "Unlocked",
    "ReviewRequested",
    "ReviewRequestedRemoved",
    "AutoMergeEnabled",
    "AutoMergeDisabled"
);
github_event!("Push");
github_event!("RegistryPackage", "Published", "Updated");
github_event!(
    "Release",
    "Published",
    "Unpublished",
    "Created",
    "Edited",
    "Deleted",
    "Prereleased",
    "Released"
);
// TODO: RepositoryDispatch events have a custom event type.
// We will need to parse these by hand instead of using the proc_macro.
github_event!("RepositoryDispatch");
// TODO: Schedule requires a Cron field. Will have to parse this by hand.
github_event!("Schedule");
github_event!("Status");
github_event!("Watch", "Started");
github_event!("WorkflowCall");
github_event!("WorkflowDispatch");
github_event!("WorkflowRun", "Completed", "Requested", "InProgress");

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowEvents {
    pub branch_protection_rule: Option<BranchProtectionRule>,
    pub check_run: Option<CheckRun>,
    pub check_suite: Option<CheckSuite>,
    pub create: Option<Create>,
    pub delete: Option<Delete>,
    pub deployment: Option<Deployment>,
    pub deployment_status: Option<DeploymentStatus>,
    pub discussion: Option<Discussion>,
    pub discussion_comment: Option<DiscussionComment>,
    pub fork: Option<Fork>,
    pub gollum: Option<Gollum>,
    pub issue_comment: Option<IssueComment>,
    pub issues: Option<Issues>,
    pub label: Option<Label>,
    pub merge_group: Option<MergeGroup>,
    pub milestone: Option<Milestone>,
    pub page_build: Option<PageBuild>,
    pub project: Option<Project>,
    pub project_card: Option<ProjectCard>,
    pub project_column: Option<ProjectColumn>,
    pub public: Option<Public>,
    pub pull_request: Option<PullRequest>,
    pub pull_request_review: Option<PullRequestReview>,
    pub pull_request_review_comment: Option<PullRequestReview>,
    pub pull_request_target: Option<PullRequestTarget>,
    pub push: Option<Push>,
    pub registry_package: Option<RegistryPackage>,
    pub release: Option<Release>,
    pub repository_dispatch: Option<RepositoryDispatch>,
    pub schedule: Option<Schedule>,
    pub status: Option<Status>,
    pub watch: Option<Watch>,
    pub workflow_call: Option<WorkflowCall>,
    pub workflow_dispatch: Option<WorkflowDispatch>,
    pub workflow_run: Option<WorkflowRun>,
}
