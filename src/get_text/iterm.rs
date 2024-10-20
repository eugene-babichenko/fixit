use std::{env, process::Command};

use super::find_command_output;

/// This is an AppleScript that gets the contents of the current session
const SCRIPT: &str = r#"
tell application "iTerm2"
    tell current session of current window
        contents
    end tell
end tell
"#;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    env::var("ITERM_SESSION_ID").ok()?;

    let output = Command::new("osascript")
        .args(["-e", SCRIPT])
        .output()
        .ok()?;

    find_command_output(cmd, output.stdout, depth)
}
