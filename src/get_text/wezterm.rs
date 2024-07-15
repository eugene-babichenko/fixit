use std::env;

use super::GetTextResult;

pub fn get_text(depth: usize) -> Option<GetTextResult> {
    env::var("WEZTERM_EXECUTABLE").ok()?;

    Some(GetTextResult {
        cmd: "wezterm",
        args: vec![
            "cli".to_string(),
            "get-text".to_string(),
            "--start-line".to_string(),
            format!("-{depth}"),
        ],
        needs_processing: true,
    })
}
