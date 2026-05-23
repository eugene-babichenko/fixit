pub fn git_missing_branch(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !error.contains("did not match any file(s) known to git") {
        return None
    }
    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "checkout")?;
    cmd.insert(idx + 1, "-b".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;
    use super::git_missing_branch;

    #[test]
    fn branch_does_not_exist() {
        let cmd = shlex("git checkout hello");
        let error = "error: pathspec 'hello' did not match any file(s) known to git";
        let expected = shlex("git checkout -b hello");
        assert_eq!(Some(expected), git_missing_branch(cmd, error));
    }

    #[test]
    fn other_error() {
        let cmd = shlex("git checkout -j hello");
        let error = "error: unknown switch `j'";
        assert!(git_missing_branch(cmd, error).is_none());
    }
}
