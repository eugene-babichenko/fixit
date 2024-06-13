use std::{env::args, fs::canonicalize, io};

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not read argv[0]")]
    Args,
    #[error("could not canonicalize the path to this executable")]
    Canononicalize(#[source] io::Error),
}

#[derive(Subcommand)]
enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl Cmd {
    pub fn run(self) -> Result<(), Error> {
        // This conversion is useful for the purposes of debugging without
        // installing the software globally every time.
        let mut executable = args().next().ok_or(Error::Args)?;
        if executable.contains('/') || executable.contains('\\') {
            executable = canonicalize(executable)
                .map_err(Error::Canononicalize)?
                .display()
                .to_string();
        };

        let name = self.name;
        // We are NOT going to use a template engine for that. It increases the
        // dependencies footprint and the amount of code for too little benefit.
        match self.shell {
            Shell::Bash => {
                print!(
                    "
function {name}() {{
    previous_cmd=\"$(fc -ln -1)\"
    fixed_cmd=\"$({executable} fix \"$previous_cmd\")\"
    if [ \"$fixed_cmd\" != \"\" ]; then
        eval \"$fixed_cmd\"
	history -s \"$fixed_cmd\"
    fi
}}
                "
                )
            }
            Shell::Fish => {
                print!(
                    "
function {name} -d \"Fix your previous command\"
    set -l previous_cmd \"$history[1]\"
    {executable} fix \"$previous_cmd\" | read -l fixed_cmd
    if [ \"$fixed_cmd\" != \"\" ]
        commandline \"$fixed_cmd\"
        commandline -f execute
    end
end
                    "
                );
            }
            Shell::Zsh => print!(
                "
function {name}() {{
    previous_cmd=\"$(fc -ln -1)\"
    fixed_cmd=\"$({executable} fix \"$previous_cmd\")\"
    if [ \"$fixed_cmd\" != \"\" ]; then
        eval \"$fixed_cmd\"
	print -s \"$fixed_cmd\"
    fi
}}
                "
            ),
        };

        Ok(())
    }
}
