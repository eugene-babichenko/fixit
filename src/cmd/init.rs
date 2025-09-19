use std::io::{stdout, Write};

use clap::{Parser, Subcommand};
use thiserror::Error;

#[derive(Parser)]
pub struct Cmd {
    // The name of the alias.
    #[arg(long, default_value = "fix")]
    name: String,
    /// The shell for which we generate the alias.
    #[command(subcommand)]
    shell: Shell,
}

#[derive(Subcommand)]
enum Shell {
    Bash,
    Fish,
    Zsh,
    Powershell,
}

#[derive(Debug, Error)]
#[error("failed to render the alias")]
pub struct Error(#[source] std::io::Error);

macro_rules! include_template {
    ($ext:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/alias.",
            $ext
        ))
    };
}

pub fn run(cmd: Cmd) -> Result<(), Error> {
    let template = match cmd.shell {
        Shell::Bash => include_template!("sh"),
        Shell::Fish => include_template!("fish"),
        Shell::Zsh => include_template!("zsh"),
        // TODO design a workflow for using this powershell contraption
        // $PSDefaultParameterValues['Out-Default:OutVariable'] = 'env:FIXIT_PREVIOUS_CMD_OUTPUT'
        // this would allow us to do quick search with powershell regardless of having a compatible
        // emulator or multiplexer
        Shell::Powershell => include_template!("ps1"),
    };
    let alias = template.replacen("__name__", &cmd.name, 1);
    stdout().write(alias.as_bytes()).map(|_| ()).map_err(Error)
}
