pub fn git_commit_no_changes(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !error.contains("no changes added to commit") {
        log::debug!("does not contain a matching error message");
        return None;
    }
    let mut res = cmd.to_vec();
    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "commit")?;
    res.insert(idx + 1, "-a".to_string());
    Some(res)
}
