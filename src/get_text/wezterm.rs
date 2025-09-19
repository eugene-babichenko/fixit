use std::{env, process::Command};

use super::find_command_output;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    if &env::var("TERM_PROGRAM").ok()? != "WezTerm" {
        return None;
    }

    let output = Command::new("wezterm")
        .args(["cli", "get-text", "--start-line", &format!("-{depth}")])
        .output()
        .ok()?;

    find_command_output(cmd, output.stdout, depth)
}
