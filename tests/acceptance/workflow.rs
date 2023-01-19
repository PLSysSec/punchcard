extern crate punchcard;

use pretty_assertions::assert_eq;
use punchcard::bindings::{load_workflow, Workflow};
use serde_yaml;

#[test]
fn parse_basic_workflow() -> Result<(), serde_yaml::Error> {
    let workflow = load_workflow("./tests/data/workflow/basic1.yml")?;
    assert!(workflow.name.is_some());
    assert!(workflow.run_name.is_some());
    println!("{workflow:?}");
    let activity_types = workflow.on.branch_protection_rule.unwrap().types.unwrap();
    assert_eq!(activity_types.len(), 2);
    Ok(())
}
