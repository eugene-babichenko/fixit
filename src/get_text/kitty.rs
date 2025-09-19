use crate::get_text::stdout_to_string;
use std::{env, process::Command};

use super::find_command_output;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    // Use specifically this variable to see if remote control is enabled.
    env::var("KITTY_PUBLIC_KEY").ok()?;

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
        .ok()?;

    if shell_integration {
        stdout_to_string(output.stdout).ok()
    } else {
        find_command_output(cmd, output.stdout, depth)
    }
}
