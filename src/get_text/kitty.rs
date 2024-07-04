use std::{env, process::Command};

use crate::get_text::{find_command_output, stdout_to_string};

pub fn get_text_kitty(cmd: &str, depth: usize) -> Option<String> {
    if env::var("KITTY_INSTALLATION_DIR").is_err() {
        return None;
    }

    log::debug!("getting the command output from kitty");

    let shell_integration = env::var("KITTY_SHELL_INTEGRATION").is_ok();
    let extent = if shell_integration {
        "last_cmd_output"
    } else {
        // TODO find a way to limit number of lines to `depth`
        "all"
    };

    let output = Command::new("kitty")
        .args(["@", "get-text", "--extent", extent])
        .output()
        .map_err(|e| log::error!("failed to get output from kitty: {e}"))
        .ok()?;

    if !output.status.success() {
        return None;
    }

    if shell_integration {
        stdout_to_string(output.stdout).ok()
    } else {
        find_command_output(cmd, output.stdout, Some(depth))
    }
}
