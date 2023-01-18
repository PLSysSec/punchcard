use clap::{arg, Command, command, ColorChoice};
use bindings::read_yaml;

mod bindings;

fn main() {
    let matches = command!()
        .author("Robbie McKinstry <robbie@pulumi.com>")
        .about("Correctness tooling for GitHub Actions")
        .version("0.1.0")
        .display_name("punchcard")
        .propagate_version(true)
        .color(ColorChoice::Auto)
        .subcommand_required(true)
        .subcommand(
            Command::new("apply")
                .about("Build and validate a workflow using the specification provided.")
                .arg_required_else_help(true)
                .arg(arg!([WORKFLOW_FILE]))
        ).get_matches();
    match matches.subcommand() {
        Some(("apply", sub_matches)) => { 
            let filepath = sub_matches.get_one::<String>("WORKFLOW_FILE").expect("File must exist").clone();
            println!("'punchcard apply' was used, file is: {:?}", filepath);
            let yaml_file = read_yaml(filepath.into());
            println!("{yaml_file}");            
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
