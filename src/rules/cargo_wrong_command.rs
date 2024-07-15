use regex::Regex;

pub fn cargo_wrong_command(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"cargo".to_string()) {
        return None;
    }

    let regex = Regex::new(r"no\ssuch\scommand:\s`([a-zA-Z-]+)`").unwrap();
    let old_cmd = regex.captures(error)?.get(1)?.as_str();
    let (old_cmd_idx, _) = cmd.iter().enumerate().find(|(_, cmd)| *cmd == old_cmd)?;

    let regex = Regex::new(r"did\syou\smean\s`([a-zA-Z-]+)`").unwrap();
    cmd[old_cmd_idx] = regex.captures(error)?.get(1)?.as_str().to_string();
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::cargo_wrong_command;

    #[test]
    fn cargo_wrong_command_test() {
        let cmd = shlex("cargo instal");
        let error = "error: no such command: `instal`

        Did you mean `install`?

        View all installed commands with `cargo --list`
        Find a package to install `instal` with `cargo search cargo-instal`";
        let expected = shlex("cargo install");
        assert_eq!(
            expected,
            cargo_wrong_command(cmd, &error.to_string().to_lowercase()).unwrap()
        );
    }
}
