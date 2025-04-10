use std::{env, io, string::FromUtf8Error};

use clap::Parser;
use regex::Regex;
use thiserror::Error;

mod iterm;
mod kitty;
mod macos_terminal;
mod rerun_command;
mod tmux;
mod wezterm;
mod zellij;

#[derive(Parser)]
pub struct Config {
    /// Enable searching via WezTerm API
    #[arg(long, env = "FIXIT_QUICK_ENABLE", default_value_t = true)]
    quick: bool,
    /// The number of lines to scan from the scrollback buffer.
    #[arg(long, env = "FIXIT_QUICK_SEARCH_DEPTH", default_value_t = 1000)]
    depth: usize,
    /// Reliably check if running inside a Powershell session
    #[arg(long, default_value_t = false)]
    powershell: bool,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("no $SHELL variable was found")]
    Shell(#[from] env::VarError),
    #[error("could not re-run the failing command (might be a problem with the $SHELL variable)")]
    ReRun(#[from] io::Error),
    #[error("the output of the command is not a valid UTF-8 text")]
    CmdOutput(#[from] FromUtf8Error),
}

pub fn get_text(config: Config, cmd: &str) -> Result<Option<Vec<String>>, Error> {
    if config.quick {
        // Terminal multiplexers go first. Everything must go in the alphanumeric order.
        let get_text = [
            tmux::get_text,
            kitty::get_text,
            zellij::get_text,
            wezterm::get_text,
            iterm::get_text,
            macos_terminal::get_text,
        ];

        for get_text in get_text {
            if let Some(command_output) = get_text(cmd, config.depth) {
                log::debug!("got fast output");
                return Ok(Some(vec![command_output]));
            }
        }

        log::debug!("quick search failed");
    } else {
        log::debug!("quick search is disabled");
    }

    rerun_command::rerun_command(cmd, config.powershell)
}

pub fn stdout_to_string(stdout: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(stdout).map_err(Into::into)
}

pub fn find_command_output(cmd: &str, stdout: Vec<u8>, depth: usize) -> Option<String> {
    if stdout.is_empty() {
        return None;
    }

    let stdout = stdout_to_string(stdout)
        .map_err(|e| log::debug!("failed to stringify the command output: {e}"))
        .ok()?;

    log::debug!("stdout {}", stdout);

    let cmd = cmd.trim();

    if !stdout.contains(cmd) {
        log::debug!("command not found in stdout");
        return None;
    }

    // FIXME This is a really shitty heuristic to find a line containing the
    // last command and to not break on messages like "command not found: git".
    // Ideally we should acknowledge OSC 133 sequences and this should result in
    // a much faster and more reliable search, but I haven't been successful at
    // extracting them out of `wezterm cli get-text --escapes` output. Ideally,
    // we get the functionality to get the output of the last command a-la kitty
    // someday.

    let stdout: Vec<_> = stdout.lines().map(|s| s.trim_end()).collect();

    let fish_error_highlight_regex = Regex::new(r"\^~+\^").unwrap();

    let mut first_res_line = 0;
    for i in (0..stdout.len()).rev().take(depth) {
        if !stdout[i].ends_with(cmd)
            || stdout[i].ends_with(&[": ", cmd].concat())
            || stdout
                .get(i + 1)
                .is_some_and(|s| fish_error_highlight_regex.is_match(s))
        {
            first_res_line = i;
        } else {
            return Some(stdout[first_res_line..].join("\n"));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::find_command_output;

    #[test]
    fn command_output_empty() {
        let cmd = "ls";
        assert_eq!(None, find_command_output(cmd, vec![], usize::MAX));
    }

    #[test]
    fn command_output_finder() {
        let output = "
fixit on ? master [$?!?] is ?? v0.1.0-alpha via ?? v1.78.0
? gti
fish: Unknown command: gti";
        let expected = "fish: Unknown command: gti".to_string();
        let cmd = "gti";
        assert_eq!(
            expected,
            find_command_output(cmd, output.as_bytes().to_vec(), usize::MAX).unwrap()
        );
    }

    #[test]
    fn command_output_clipped() {
        let output = "fish: Unknown command: gti";
        let cmd = "gti";
        assert_eq!(
            None,
            find_command_output(cmd, output.as_bytes().to_vec(), usize::MAX)
        );
    }

    #[test]
    fn command_output_fish() {
        let output = "
fixit on ? master [$?!?] is ?? v0.1.0-alpha via ?? v1.78.0
? time gti push
fish: Unknown command: gti
fish:
time gti push
     ^~^";
        let expected = "fish: Unknown command: gti
fish:
time gti push
     ^~^";
        let cmd = "time gti push";
        assert_eq!(
            expected,
            find_command_output(cmd, output.as_bytes().to_vec(), usize::MAX).unwrap()
        );
    }

    #[test]
    fn no_relevant_command() {
        let output = "
fixit on ? master [??] is ?? v0.3.1-beta via ?? v1.78.0
? thefuck
Traceback (most recent call last):
  File \"/Users/eugene/Library/Python/3.8/bin/thefuck\", line 5, in <module>
    from thefuck.entrypoints.main import main
ModuleNotFoundError: No module named 'thefuck'

fixit on ? master [??] is ?? v0.3.1-beta via ?? v1.78.0
? help

fixit on ? master [??] is ?? v0.3.1-beta via ?? v1.78.0
? yay
fish: Unknown command: yay
            ";
        let cmd = "time gti push";
        assert_eq!(
            None,
            find_command_output(cmd, output.as_bytes().to_vec(), usize::MAX)
        )
    }

    #[test]
    fn similar_commands() {
        let output = "fixit on  master [!?] is 📦 v0.4.0 via 🦀 v1.78.0 took 8s
❯ cp ./target/debug/fixit /bin
cp: /bin/fixit: Operation not permitted

fixit on  master [!?] is 📦 v0.4.0 via 🦀 v1.78.0
❯ cp ./target/debug/fixit
usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file
       cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory";
        let cmd = "cp ./target/debug/fixit";
        let expected =
            "usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file
       cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory";
        assert_eq!(
            Some(expected.to_string()),
            find_command_output(cmd, output.as_bytes().to_vec(), usize::MAX)
        );
    }
}
