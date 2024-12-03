pub fn cargo_clippy_args(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"clippy".to_string()) {
        return None;
    }

    let r = regex::Regex::new(r#"unexpected argument '(-[wadf]{1})'"#).unwrap();
    let c = r.captures(error)?;
    let arg = c.get(1)?.as_str().to_uppercase();
    let pos = cmd.iter().enumerate().find(|(_, x)| **x == arg)?.0;

    cmd.remove(pos);
    let arg2 = cmd.remove(pos);

    if !cmd.contains(&"--".to_string()) {
        cmd.push("--".to_string());
    }

    cmd.push(arg);
    cmd.push(arg2.to_string());

    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    #[test]
    fn cargo_clippy_args_test() {
        let cmd = shlex("cargo clippy -D warnings");
        let error = "error: unexpected argument '-D'".to_lowercase();
        let expected = shlex("cargo clippy -- -D warnings");
        assert_eq!(Some(expected), super::cargo_clippy_args(cmd, &error));
    }
}
