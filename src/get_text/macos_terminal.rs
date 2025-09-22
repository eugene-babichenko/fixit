use crate::get_text::quick_search_generic;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    quick_search_generic(
        "Apple_Terminal",
        &[
            "osascript",
            "-e",
            r#"tell application "Terminal" to get history of selected tab of front window"#,
        ],
        depth,
        cmd,
    )
}
