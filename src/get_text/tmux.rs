use crate::get_text::find_command_output;
use std::{env, process::Command};

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    if &env::var("TERM_PROGRAM").ok()? != "tmux" {
        return None;
    }

    log::debug!("getting text from tmux");

    let output = Command::new("tmux")
        .args(["capture-pane", "-p", "-S", &format!("-{depth}")])
        .output()
        .ok()?;

    log::debug!("got tmux output");

    find_command_output(cmd, output.stdout, depth)
}
