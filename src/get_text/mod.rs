use std::{env, io, string::FromUtf8Error};

use clap::Parser;
use itertools::Itertools;
use regex::Regex;
use thiserror::Error;

mod kitty;
mod rerun_command;
mod tmux;
mod wezterm;

#[derive(Parser)]
pub struct Config {
    /// Enable searching via WezTerm API
    #[arg(env = "FIXIT_QUICK_ENABLE", default_value_t = true)]
    quick: bool,
    /// The number of lines to scan from the WezTerm scrollback buffer.
    #[arg(env = "FIXIT_QUICK_SEARCH_DEPTH", default_value_t = 1000)]
    depth: usize,
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
        if let Some(output) = tmux::get_text_tmux(cmd, config.depth) {
            log::debug!("got output from tmux");
            return Ok(Some(vec![output]));
        }

        // Then we look through supported terminal emulators.
        if let Some(output) = kitty::get_text_kitty(cmd, config.depth) {
            log::debug!("got output from kitty");
            return Ok(Some(vec![output]));
        }
        if let Some(output) = wezterm::get_text_wezterm(cmd, config.depth) {
            log::debug!("got output from wezterm");
            return Ok(Some(vec![output]));
        }
    }

    rerun_command::rerun_command(cmd).map(|maybe_output| maybe_output.map(|(a, b)| vec![a, b]))
}

pub fn stdout_to_string(stdout: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(stdout).map_err(Into::into)
}

pub fn find_command_output(cmd: &str, stdout: Vec<u8>, depth: Option<usize>) -> Option<String> {
    let depth = depth.unwrap_or(usize::MAX);

    let stdout = stdout_to_string(stdout)
        .map_err(|e| log::debug!("failed to stringify the command output: {e}"))
        .ok()?;

    if !stdout.contains(cmd) {
        log::debug!("command not found in stdout");
        return None;
    }

    let fish_error_highlight_regex = Regex::new(r"\^[~]+\^").unwrap();

    // FIXME This is a really shitty heuristic to find a line containing the
    // last command and to not break on messages like "command not found: git".
    // Ideally we should acknowledge OSC 133 sequences and this should result in
    // a much faster and more reliable search, but I haven't been successful at
    // extracting them out of `wezterm cli get-text --escapes` output. Ideally,
    // we get the functionality to get the output of the last command a-la kitty
    // someday.
    let stdout: Vec<&str> = stdout.lines().rev().collect();
    let mut res: Vec<&str> = stdout
        .iter()
        .rev()
        .take(depth)
        .circular_tuple_windows::<(_, _)>()
        .take_while(|(s_prev, s_curr)| {
            // fish errors for complex commands contain a light highlighting the
            // exact command that failed:
            // time qwerty
            //      ^~~~~^
            // which this algo can confuse for an actual command.
            !fish_error_highlight_regex.is_match(s_prev)
                || !s_curr.ends_with(cmd)
                || s_curr.ends_with(&[": ", cmd].concat())
        })
        .map(|s| *s.1)
        .collect();
    res.reverse();
    Some(res.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::find_command_output;

    #[test]
    fn test_command_output_finder() {
        let output = "
fixit on ? master [$?!?] is ?? v0.1.0-alpha via ?? v1.78.0
? gti
fish: Unknown command: gti";
        let expected = "fish: Unknown command: gti".to_string();
        let cmd = "gti";
        assert_eq!(
            expected,
            find_command_output(cmd, output.to_string().into_bytes(), Some(1000)).unwrap()
        );
    }
}
