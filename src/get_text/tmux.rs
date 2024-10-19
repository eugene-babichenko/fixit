use std::env;

use super::GetTextCommand;

pub fn get_text(depth: usize) -> Option<GetTextCommand> {
    env::var("TMUX").ok()?;

    Some(GetTextCommand {
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
