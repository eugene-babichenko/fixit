use std::{env, process::Command};

use crate::get_text::find_command_output;

pub fn get_text_tmux(cmd: &str, depth: usize) -> Option<String> {
    env::var("TMUX").ok()?;

    log::debug!("getting the command output from tmux");

    let output = Command::new("tmux")
        .args(["capture-pane", "-p", "-S", &format!("-{}", depth)])
        .output()
        .map_err(|e| log::error!("failed to get output from tmux: {e}"))
        .ok()?;

    if !output.status.success() {
        return None;
    }

    find_command_output(cmd, output.stdout, None)
}
