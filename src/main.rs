#![deny(warnings)]

use std::{error::Error as _, process::exit};

use clap::{Parser, Subcommand};
use cmd::{fix, init};
use thiserror::Error;

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

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Fix(#[from] cmd::fix::Error),
    #[error(transparent)]
    Init(#[from] cmd::init::Error),
}

fn run() -> Result<(), Error> {
    env_logger::init();

    let args = Args::parse();

    match args.command {
        Commands::Fix(cmd) => fix::run(cmd).map_err(Into::into),
        Commands::Init(cmd) => init::run(cmd).map_err(Into::into),
    }
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
