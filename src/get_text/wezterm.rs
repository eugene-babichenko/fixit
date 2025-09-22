use crate::get_text::quick_search_generic;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    quick_search_generic(
        "WezTerm",
        &[
            "wezterm",
            "cli",
            "get-text",
            "--start-line",
            &format!("-{depth}"),
        ],
        depth,
        cmd,
    )
}
