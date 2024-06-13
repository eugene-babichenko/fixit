use std::{env, process::Command};

use crate::get_text::find_command_output;

pub fn get_text_wezterm(cmd: &str, depth: usize) -> Option<String> {
    env::var("WEZTERM_EXECUTABLE").ok()?;

    log::debug!("getting the command output from WezTerm");

    let output = Command::new("wezterm")
        .args(["cli", "get-text", "--start-line", &format!("-{}", depth)])
        .output()
        .map_err(|e| log::error!("failed to get output from WezTerm: {e}"))
        .ok()?;

    if !output.status.success() {
        return None;
    }

    find_command_output(cmd, output.stdout, None)
}
