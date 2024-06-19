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

#[cfg(test)]
mod test {
    use crate::shlex::shlex;

    use super::*;

    #[test]
    fn git_commit_no_changes_test() {
        let cmd = shlex("git commit -m 'initial commit'");
        let error = "no changes added to commit (use \"git add\" and/or \"git commit -a\")";
        let expected = shlex("git commit -a -m 'initial commit'");
        assert_eq!(Some(expected), git_commit_no_changes(&cmd, error));
    }
}
