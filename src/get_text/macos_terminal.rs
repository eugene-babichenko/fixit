use std::{env, process::Command};

use super::find_command_output;

/// This is an AppleScript that gets the contents of the current tab
const SCRIPT: &str =
    r#"tell application "Terminal" to get history of selected tab of front window"#;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    if env::var("TERM_PROGRAM") != Ok("Apple_Terminal".to_string()) {
        return None;
    }

    let output = Command::new("osascript")
        .args(["-e", SCRIPT])
        .output()
        .ok()?;

    find_command_output(cmd, output.stdout, depth)
}
