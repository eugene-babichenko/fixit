use regex::Regex;

pub fn git_wrong_command(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !error.contains("not a git command") {
        log::debug!("does not contain a matching error message");
        return None;
    }

    let regex = Regex::new(r"'([a-zA-Z0-9-]+)'\sis\snot\sa\sgit\scommand").unwrap();
    let old_cmd = regex
        .captures_iter(error)
        .map(|c| c.extract::<1>().1[0])
        .next()?;
    let (old_cmd_idx, _) = cmd.iter().enumerate().find(|(_, cmd)| *cmd == old_cmd)?;

    let regex = Regex::new(r"similar\scommand\sis\s*([a-zA-Z0-9-]+)").unwrap();
    let new_cmd = regex
        .captures_iter(error)
        .map(|c| c.extract::<1>().1[0])
        .next()?;
    let mut res = cmd.to_vec();
    res[old_cmd_idx] = new_cmd.to_string();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::git_wrong_command;

    #[test]
    fn git_pusk() {
        let cmd = &["git".to_string(), "pusk".to_string()];
        let error = "git: 'pusk' is not a git command. See 'git --help'.

The most similar command is
        push";

        assert_eq!(
            Some(vec!["git".to_string(), "push".to_string()]),
            git_wrong_command(cmd, error)
        );
    }
}
