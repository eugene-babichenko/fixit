use regex::Regex;

pub fn git_wrong_command(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !error.contains("not a git command") {
        log::debug!("does not contain a matching error message");
        return None;
    }

    let regex = Regex::new(r"'([a-zA-Z0-9-]+)'\sis\snot\sa\sgit\scommand").unwrap();
    let old_cmd = regex.captures(error)?.get(1)?.as_str();
    let (old_cmd_idx, _) = cmd.iter().enumerate().find(|(_, cmd)| *cmd == old_cmd)?;

    let regex = Regex::new(r"similar\scommand\sis\s*([a-zA-Z0-9-]+)").unwrap();
    cmd[old_cmd_idx] = regex.captures(error)?.get(1)?.as_str().to_string();
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::git_wrong_command;

    #[test]
    fn git_pusk() {
        let cmd = shlex("git pusk");
        let error = "git: 'pusk' is not a git command. See 'git --help'.

The most similar command is
        push";

        assert_eq!(Some(shlex("git push")), git_wrong_command(cmd, error));
    }
}
