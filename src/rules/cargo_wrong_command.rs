use regex::Regex;

pub fn cargo_wrong_command(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"cargo".to_string()) {
        return None;
    }

    let regex = Regex::new(r"no\ssuch\scommand:\s`([a-zA-Z-]+)`").unwrap();
    let old_cmd = regex
        .captures_iter(error)
        .map(|c| c.extract::<1>().1[0])
        .next()?;
    let (old_cmd_idx, _) = cmd.iter().enumerate().find(|(_, cmd)| *cmd == old_cmd)?;

    let regex = Regex::new(r"did\syou\smean\s`([a-zA-Z-]+)`").unwrap();
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
    use super::cargo_wrong_command;

    #[test]
    fn cargo_wrong_command_test() {
        let cmd = &["cargo".to_string(), "instal".to_string()];
        let error = "error: no such command: `instal`

        Did you mean `install`?

        View all installed commands with `cargo --list`
        Find a package to install `instal` with `cargo search cargo-instal`";
        let expected = vec!["cargo".to_string(), "install".to_string()];
        assert_eq!(
            expected,
            cargo_wrong_command(cmd, &error.to_string().to_lowercase()).unwrap()
        );
    }
}
