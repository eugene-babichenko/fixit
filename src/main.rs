use std::{error::Error as _, process::exit};

use clap::{Parser, Subcommand};
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
    setup_logger();

    let args = Args::parse();

    log::debug!("using {} threads", rayon::current_num_threads());

    match args.command {
        Commands::Fix(cmd) => cmd.run().map_err(Into::into),
        Commands::Init(cmd) => cmd.run().map_err(Into::into),
    }
}

fn setup_logger() {
    use env_logger::{Builder, Env};

    let env = Env::new()
        .filter("FIXIT_LOG")
        .write_style("FIXIT_LOG_STYLE");
    Builder::from_env(env).init();
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
