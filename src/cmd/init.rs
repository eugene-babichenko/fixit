use clap::{Parser, Subcommand};

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
}

impl Cmd {
    pub fn run(self) {
        let name = self.name;
        // We are NOT going to use a template engine for that. It increases the
        // dependencies footprint and the amount of code for too little benefit.
        match self.shell {
            Shell::Bash => {
                print!(
                    r#"
function {name}() {{
    previous_cmd="$(fc -ln -1)"
    export FIXIT_FNS="$(
        declare -F | cut -d' ' -f3
        alias | cut -d'=' -f1
    )"
    fixed_cmd="$(fixit fix "$previous_cmd")"
    if [ "$fixed_cmd" != "" ]; then
        eval "$fixed_cmd"
	history -s "$fixed_cmd"
    fi
}}"#
                )
            }
            Shell::Fish => {
                print!(
                    r#"
function {name} -d "Fix your previous command"
    set -l previous_cmd "$history[1]"
    set -lx FIXIT_FNS (
        functions | cut -d' ' -f1
        alias | cut -d' ' -f2
    )
    fixit fix "$previous_cmd" | read -l fixed_cmd
    if [ "$fixed_cmd" != "" ]
        commandline "$fixed_cmd"
        commandline -f execute
    end
end
                    "#
                );
            }
            Shell::Zsh => print!(
                r#"
function {name}() {{
    previous_cmd="$(fc -ln -1)"
    export FIXIT_FNS="$(
        print -l ${{(ok)functions}}
        alias | cut -d'=' -f1
    )"
    fixed_cmd="$(fixit fix "$previous_cmd")"
    if [ "$fixed_cmd" != "" ]; then
        eval "$fixed_cmd"
	print -s "$fixed_cmd"
    fi
}}"#
            ),
        };
    }
}
