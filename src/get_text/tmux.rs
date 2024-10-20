use crate::get_text::find_command_output;
use std::{env, process::Command};

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    env::var("TMUX").ok()?;

    let output = Command::new("tmux")
        .args(["capture-pane", "-p", "-S", &format!("-{}", depth)])
        .output()
        .ok()?;

    find_command_output(cmd, output.stdout, depth)
}
