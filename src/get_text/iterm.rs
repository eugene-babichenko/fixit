use std::env;

use super::GetTextResult;

/// This is an AppleScript that gets the contents of the current session
const SCRIPT: &str = r#"
tell application "iTerm2"
    tell current session of current window
        contents
    end tell
end tell
"#;

pub fn get_text(_depth: usize) -> Option<GetTextResult> {
    env::var("ITERM_SESSION_ID").ok()?;

    Some(GetTextResult {
        cmd: "osascript",
        args: vec!["-e".to_string(), SCRIPT.to_string()],
        needs_processing: true,
    })
}
