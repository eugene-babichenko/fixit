use regex::Regex;

pub fn uv_unexpected_argument(mut cmd: Vec<String>, err: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"uv".to_string()) {
        return None;
    }

    let regex = Regex::new(r"unexpected\sargument\s'([a-zA-Z-]+)'\sfound").unwrap();
    let old_argument = regex.captures(err)?.get(1)?.as_str();
    let (old_argument_idx, _) = cmd
        .iter()
        .enumerate()
        .find(|(_, cmd)| *cmd == old_argument)?;

    let regex = Regex::new(r"similar\sargument\sexists:\s'([a-zA-Z-]+)'").unwrap();
    cmd[old_argument_idx] = regex.captures(err)?.get(1)?.as_str().to_string();
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::shlex::shlex;

    #[test]
    fn fix_argument() {
        let cmd = shlex("uv run --hlp");
        let err = r#"error: unexpected argument '--hlp' found

  tip: a similar argument exists: '--help'

Usage: uv run --help

For more information, try '--help'."#;
        let expected = shlex("uv run --help");
        assert_eq!(Some(expected), uv_unexpected_argument(cmd, err));
    }

    #[test]
    fn unrelated_error() {
        let cmd = shlex("uv rn");
        let err = r#"error: unrecognized subcommand 'rn'

Usage: uv [OPTIONS] <COMMAND>

For more information, try '--help'."#;
        assert_eq!(None, uv_unexpected_argument(cmd, err));
    }

    #[test]
    fn unrelated_command() {
        let cmd = shlex("cargo tst");
        let err = r#"error: no such command: `tst`

        Did you mean `test`?"#;
        assert_eq!(None, uv_unexpected_argument(cmd, err));
    }
}
