use std::env;

use super::GetTextCommand;

pub fn get_text(_depth: usize) -> Option<GetTextCommand> {
    env::var("KITTY_INSTALLATION_DIR").ok()?;

    let shell_integration = env::var("KITTY_SHELL_INTEGRATION").is_ok();
    let extent = if shell_integration {
        "last_cmd_output"
    } else {
        // TODO find a way to limit number of lines to `depth`
        "all"
    };

    Some(GetTextCommand {
        cmd: "kitty",
        args: vec![
            "@".to_string(),
            "get-text".to_string(),
            "--extent".to_string(),
            extent.to_string(),
        ],
        needs_processing: !shell_integration,
    })
}
