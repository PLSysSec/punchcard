extern crate punchcard;

use punchcard::bindings::{read_yaml, WorkflowEvents};
use pretty_assertions::assert_eq;
use serde_yaml;

#[test]
fn parse_branch_protection() -> Result<(), serde_yaml::Error> {
    let filename = "./tests/data/events/branch1.yml";
    let loaded = read_yaml(filename.into());
    let result: WorkflowEvents = serde_yaml::from_str(&loaded)?;
    println!("{result:?}");
    let activity_types = result
        .branch_protection_rule
        .unwrap()
        .types
        .unwrap();
    assert_eq!(activity_types.len(), 2);    
    Ok(())
}
