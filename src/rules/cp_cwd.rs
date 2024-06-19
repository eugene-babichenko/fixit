pub fn cp_cwd(cmd: &[String], _error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"cp".to_string()) || !cmd.len() == 2 {
        return None;
    }

    let mut cmd = cmd.to_vec();
    cmd.push(".".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::cp_cwd;

    #[test]
    fn cp_cwd_success() {
        let cmd = shlex("cp target/release/fixit");
        let error = "";
        let expected = shlex("cp target/release/fixit .");
        assert_eq!(Some(expected), cp_cwd(&cmd, error));
    }

    #[test]
    pub fn non_cp_cmd() {
        let cmd = &["git".to_string(), "push".to_string()];
        let error = "";
        assert_eq!(None, cp_cwd(cmd, error));
    }
}
