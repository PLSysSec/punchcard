extern crate punchcard;

use glob::glob;
use pretty_assertions::assert_eq;
use punchcard::bindings::load_workflow;
use serde_yaml;
use std::path::PathBuf;

#[test]
fn parse_basic_workflow() -> Result<(), serde_yaml::Error> {
    let workflow = load_workflow("./tests/data/workflow/basic1.yml")?;
    assert!(workflow.name.is_some());
    assert!(workflow.run_name.is_some());
    let activity_types = workflow.on.branch_protection_rule.unwrap().types.unwrap();
    assert_eq!(activity_types.len(), 2);
    Ok(())
}

#[test]
fn parse_pulumi_ok() -> Result<(), serde_yaml::Error> {
    let glob_pattern = "./tests/data/workflow/pulumi/*.yml";
    let files: Vec<PathBuf> = glob(glob_pattern)
        .expect("Failed to read glob pattern")
        .into_iter()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    for file in files {
        let displayable = file.clone();
        let debug = displayable.display();
        let wkflow = load_workflow(file);
        assert!(wkflow.is_ok(), "Path: {debug}\nError: {}", wkflow.unwrap_err());
    }
    Ok(())
}
