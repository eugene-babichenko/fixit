use crate::get_text::quick_search_generic;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    quick_search_generic(
        "tmux",
        &["tmux", "capture-pane", "-p", "-S", &format!("-{depth}")],
        depth,
        cmd,
    )
}
