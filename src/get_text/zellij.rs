use std::env;

use super::quick_search_generic_no_term;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    env::var("ZELLIJ").ok()?;
    log::debug!("getting text from zellij");
    let res =
        quick_search_generic_no_term(&["zellij", "action", "dump-screen", "--full"], depth, cmd)?;
    log::debug!("got zellij output");
    Some(res)
}
