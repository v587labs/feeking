
use clap::{SubCommand};

pub fn run_subcommand() -> SubCommand {
    SubCommand::with_name("run")
        .about("This is my subcommand")
}