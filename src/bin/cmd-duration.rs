use cmd_duration_rs::{CmdDurationService, Result};
use log::LevelFilter;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "duration",
    about = "A Rust tool to time command execution",
    version = "1.0.0"
)]
struct Opt {
    /// The command to run
    command: String,

    /// Arguments for the command
    args: Vec<String>,
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let opt = Opt::from_args();

    if let Err(err) = run(opt) {
        eprintln!("{}", err);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    let mut service = CmdDurationService::new(opt.command, opt.args);
    service.run();

    Ok(())
}
