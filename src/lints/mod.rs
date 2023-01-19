use std::collections::BTreeMap;
use crate::bindings::Workflow;

pub trait Lint {
    fn name(&self) -> &str;
    fn is_preset(&self) -> bool {
        false
    }
    fn lint(&self, collection: WorkflowCollection) -> Vec<Message>;
}

pub struct WorkflowCollection {
    // These are the files located in /.github/workflows/ directory.
    files: BTreeMap<String, Workflow>,
}

pub struct Message {
    level: LintLevel,
    body: String,
}

pub enum LintLevel {
    Info,
    Warning,
    Error,
} 
