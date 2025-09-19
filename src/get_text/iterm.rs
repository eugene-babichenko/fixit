use std::{env, process::Command};

use super::find_command_output;

/// This is an AppleScript that gets the contents of the current session
const SCRIPT: &str =
    r#"tell application "iTerm2" to get contents of current session of current window"#;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    if &env::var("TERM_PROGRAM").ok()? != "iTerm.app" {
        return None;
    }

    let output = Command::new("osascript")
        .args(["-e", SCRIPT])
        .output()
        .ok()?;

    find_command_output(cmd, output.stdout, depth)
}
