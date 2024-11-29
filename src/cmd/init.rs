use std::io::stdout;

use clap::{Parser, Subcommand};
use rinja::Template;
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

#[derive(Template)]
#[template(path = "alias.sh", escape = "none")]
struct BashTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "alias.fish", escape = "none")]
struct FishTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "alias.zsh", escape = "none")]
struct ZshTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "alias.ps1", escape = "none")]
struct PowershellTemplate {
    name: String,
}

fn render_template<T: Template>(t: T) -> Result<(), Error> {
    t.write_into(&mut stdout()).map_err(Error)
}

pub fn run(cmd: Cmd) -> Result<(), Error> {
    let name = cmd.name;
    match cmd.shell {
        Shell::Bash => render_template(BashTemplate { name }),
        Shell::Fish => render_template(FishTemplate { name }),
        Shell::Zsh => render_template(ZshTemplate { name }),
        // TODO design a workflow for using this powershell contraption
        // $PSDefaultParameterValues['Out-Default:OutVariable'] = 'env:FIXIT_PREVIOUS_CMD_OUTPUT'
        // this would allow us to do quick search with powershell regardless of having a compatible
        // emulator or multiplexer
        Shell::Powershell => render_template(PowershellTemplate { name }),
    }
}
