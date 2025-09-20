#![deny(warnings)]

use std::process::exit;

use anyhow::Result;
use clap::{Parser, Subcommand};
use cmd::{fix, init};

mod cmd;
mod get_text;
mod rules;
mod shlex;

/// A command line utility that fixes mistakes in your previous command.
///
/// More info: https://github.com/eugene-babichenko/fixit
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Fix(cmd::fix::Cmd),
    Init(cmd::init::Cmd),
}

fn run() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    match args.command {
        Commands::Fix(cmd) => fix::run(cmd)?,
        Commands::Init(cmd) => init::run(cmd)?,
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");

        let mut source = err.source();
        while let Some(err) = source {
            eprintln!("|-> {err}");
            source = err.source();
        }

        exit(1)
    }
}
