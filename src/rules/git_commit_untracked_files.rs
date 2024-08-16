pub fn git_commit_untracked_files(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !(error.contains("no changes added to commit") || error.contains("untracked files present"))
    {
        log::debug!("does not contain a matching error message");
        return None;
    }
    let mut new_cmd = vec![
        "git".to_string(),
        "add".to_string(),
        "-A".to_string(),
        "&&".to_string(),
    ];
    new_cmd.append(&mut cmd);
    Some(new_cmd)
}

#[cfg(test)]
mod test {
    use crate::shlex::shlex;

    use super::*;

    #[test]
    fn git_commit_no_changes_test() {
        let cmd = shlex("git commit -m 'initial commit'");
        let error = "no changes added to commit (use \"git add\" and/or \"git commit -a\")";
        let expected = shlex("git add -A && git commit -m 'initial commit'");
        assert_eq!(Some(expected), git_commit_untracked_files(cmd, error));
    }

    #[test]
    fn git_commit_untracked_files_test() {
        let cmd = shlex("git commit -m 'initial commit'");
        let error = "nothing added to commit but untracked files present";
        let expected = shlex("git add -A && git commit -m 'initial commit'");
        assert_eq!(Some(expected), git_commit_untracked_files(cmd, error));
    }
}
