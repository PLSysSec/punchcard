use clap::{arg, command, ColorChoice, Command};
use emojis::get_by_shortcode;
use fern::colors::{Color, ColoredLevelConfig};
use log::{info, warn, Level, LevelFilter};
use punchcard::bindings::{load_workflow, Workflow};
use punchcard::cmd::initialize;
use std::sync::Once;

static LOGGER_READY: Once = Once::new();

fn main() {
    setup_logger(LevelFilter::Info);

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
                .arg(arg!([WORKFLOW_FILE])),
        )
        .subcommand(
            Command::new("init").about("Initialize Punchcard configuration for this directory."),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("apply", sub_matches)) => {
            let filepath = sub_matches
                .get_one::<String>("WORKFLOW_FILE")
                .expect("File must exist")
                .clone();
            lint_file(filepath);
        }
        Some(("init", _)) => {
            initialize();
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

fn setup_logger(level: log::LevelFilter) {
    LOGGER_READY.call_once(|| {
        if let Err(err) = init_fern_logger(level) {
            println!("Error encounter when creating logger: {}", err);
            panic!("{}", err);
        }
    });
}

fn init_fern_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::BrightYellow)
        .info(Color::BrightBlue)
        .debug(Color::BrightGreen)
        .trace(Color::BrightWhite);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            let event_level = &record.level();
            let emoji = select_emoji(event_level);
            let color_code = colors.get_color(event_level);
            let color = color_code.to_fg_str();
            let color_terminator = "\x1B[0m";
            out.finish(format_args!(
                "{emoji} {color_start}{message}{color_end}",
                color_start = format_args!("\x1B[{}m", color),
                color_end = color_terminator,
                message = message,
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn select_emoji(level: &Level) -> String {
    let shortcode = match level {
        Level::Error => "warning",
        Level::Warn => "exclamation",
        Level::Info => "notebook",
        Level::Debug => "bug",
        Level::Trace => "pencil2",
    };
    get_by_shortcode(shortcode).unwrap().to_string()
}

// TODO: Turn this into a pipeline.
// TODO: Collect lint warnings together with a channel. Count the warnings before
//       printing them to the screen.
// TODO: Use minette to colorize errors and provide more context, like Span information?
fn lint_file(file: String) {
    info!("Loading file {file:?}");
    let workflow = load_workflow(&file).expect("YAML must be valid");
    check_name(&workflow);
    check_run_name(&workflow);
    info!("Linting complete.");
}

fn check_name(workflow: &Workflow) {
    if workflow.name.is_none() {
        warn!("Your workflow does not have an explicit name. It's a best practice to provide a human-readable name, as this name will appear in the GitHub UI. By default, the file path is used, which can be harder to understand at a glance than a more informative name.");
    }
}

fn check_run_name(workflow: &Workflow) {
    if workflow.run_name.is_none() {
        warn!("Your workflow does not set an explicit 'run-name'. It's a best practice to provide a human-readable run-name using action inputs to categorize similar runs together.");
    }
}
