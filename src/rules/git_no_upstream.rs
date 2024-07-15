pub fn git_no_upstream(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    let regex =
        regex::Regex::new(r"--set-upstream\s([A-Za-z0-9-_./]+)\s([A-Za-z0-9-_./]+)").unwrap();
    let origin = regex.captures(error)?;
    let (idx, _) = cmd.iter().enumerate().find(|(_, s)| *s == "push")?;
    cmd.insert(idx + 1, "--set-upstream".to_string());
    cmd.insert(idx + 2, origin.get(1)?.as_str().to_string());
    cmd.insert(idx + 3, origin.get(2)?.as_str().to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::git_no_upstream;

    #[test]
    fn test_no_upstream() {
        let cmd = shlex("git push");
        let error = "fatal: The current branch master has no upstream branch.
        To push the current branch and set the remote as upstream, use

            git push --set-upstream origin master

        To have this happen automatically for branches without a tracking
        upstream, see 'push.autoSetupRemote' in 'git help config'.";
        assert_eq!(
            Some(shlex("git push --set-upstream origin master")),
            git_no_upstream(cmd, &error.to_lowercase())
        );
    }
}
