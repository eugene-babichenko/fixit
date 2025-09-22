use crate::get_text::quick_search_generic;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    quick_search_generic(
        "iTerm.app",
        &[
            "osascript",
            "-e",
            r#"tell application "iTerm2" to get contents of current session of current window"#,
        ],
        depth,
        cmd,
    )
}
