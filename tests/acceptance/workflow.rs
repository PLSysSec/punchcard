extern crate punchcard;

use punchcard::bindings::{read_yaml, Workflow};
use pretty_assertions::assert_eq;
use serde_yaml;

fn load_workflow(file: &'static str) -> Result<Workflow, serde_yaml::Error> {
    let loaded = read_yaml(file.into());
    let result: Workflow = serde_yaml::from_str(&loaded)?;
    Ok(result)
}

#[test]
fn parse_basic_workflow() -> Result<(), serde_yaml::Error> {
    let workflow = load_workflow("./tests/data/workflow/basic1.yml")?;
    assert!(workflow.name.is_some());
    assert!(workflow.run_name.is_some());
    println!("{workflow:?}");
    let activity_types = workflow.on
        .branch_protection_rule
        .unwrap()
        .types
        .unwrap();
    assert_eq!(activity_types.len(), 2);
    Ok(())
}
