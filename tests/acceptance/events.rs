extern crate punchcard;

use pretty_assertions::assert_eq;
use punchcard::bindings::{read_yaml, WorkflowEvents};
use serde_yaml;

fn load_event(file: &'static str) -> Result<WorkflowEvents, serde_yaml::Error> {
    let loaded = read_yaml(file);
    let result: WorkflowEvents = serde_yaml::from_str(&loaded)?;
    Ok(result)
}

#[test]
fn parse_branch_protection() -> Result<(), serde_yaml::Error> {
    let event = load_event("./tests/data/events/branch1.yml")?;
    let activity_types = event.branch_protection_rule.unwrap().types.unwrap();
    assert_eq!(activity_types.len(), 2);
    Ok(())
}

#[test]
fn parse_check_run() -> Result<(), serde_yaml::Error> {
    let event = load_event("./tests/data/events/check_run1.yml")?;
    let activity_types = event.check_run.unwrap().types.unwrap();
    assert_eq!(activity_types.len(), 4);
    Ok(())
}

#[test]
fn parse_check_suite() -> Result<(), serde_yaml::Error> {
    let event = load_event("./tests/data/events/check_suite1.yml")?;
    let activity_types = event.check_suite.unwrap().types.unwrap();
    assert_eq!(activity_types.len(), 1);
    Ok(())
}

#[test]
fn parse_create() -> Result<(), serde_yaml::Error> {
    let event = load_event("./tests/data/events/create1.yml")?;
    assert!(event.create.is_some());
    Ok(())
}

#[test]
fn parse_delete() -> Result<(), serde_yaml::Error> {
    let event = load_event("./tests/data/events/delete1.yml")?;
    assert!(event.delete.is_some());
    Ok(())
}
