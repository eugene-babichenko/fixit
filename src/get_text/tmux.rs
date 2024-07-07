use std::env;

use super::GetTextResult;

pub fn get_text(depth: usize) -> Option<GetTextResult> {
    env::var("TMUX").ok()?;

    Some(GetTextResult {
        cmd: "tmux",
        args: vec![
            "capture-pane".to_string(),
            "-p".to_string(),
            "-S".to_string(),
            format!("-{}", depth),
        ],
        needs_processing: true,
    })
}
