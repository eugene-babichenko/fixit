use std::env;

use super::GetTextCommand;

pub fn get_text(depth: usize) -> Option<GetTextCommand> {
    env::var("WEZTERM_EXECUTABLE").ok()?;

    Some(GetTextCommand {
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
