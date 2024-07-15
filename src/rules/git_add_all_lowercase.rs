pub fn git_add_all_lowercase(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"git".to_string())
        || !cmd.contains(&"add".to_string())
        || !error.contains(&"unknown switch".to_string())
    {
        return None;
    }

    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "-a")?;
    cmd[idx] = "-A".to_string();
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::git_add_all_lowercase;

    #[test]
    fn git_add_all_lowercase_test() {
        let cmd = shlex("git add -a");
        let error = "error: unknown switch `a'";
        let expected = shlex("git add -A");
        assert_eq!(expected, git_add_all_lowercase(cmd, error).unwrap());
    }
}
