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
    Powershell,
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
            // TODO design a workflow for using this powershell contraption
            // $PSDefaultParameterValues['Out-Default:OutVariable'] = 'env:FIXIT_PREVIOUS_CMD_OUTPUT'
            // this would allow us to do quick search with powershell regardless of having a compatible
            // emulator or multiplexer
            Shell::Powershell => print!(
                r#"
function {name} {{
    $previousCmd = (Get-History -Count 1).CommandLine
    $env:FIXIT_FNS = (Get-Command).Name
    # trimming is required to make Add-History work
    $fixedCmd = (fixit fix --powershell "$previousCmd" | Out-String).Trim()
    if ( $fixedCmd -ne '' ) {{
        $startTime = Get-Date
        Invoke-Expression $fixedCmd
        $status = if ($?) {{ "Completed" }} else {{ "Failed" }}
        $endTime = Get-Date
        $history = [pscustomobject]@{{
            CommandLine = $fixedCmd
            ExecutionStatus = $status
            StartExecutionTime = $startTime
            EndExecutionTime = $endTime
        }}
        Add-History -InputObject @($history)
    }}
}}"#,
            ),
        };
    }
}
