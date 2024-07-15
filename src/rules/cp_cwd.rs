pub fn cp_cwd(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"cp".to_string()) || !(error.contains("usage") || error.contains("operand")) {
        return None;
    }

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
        let error = "usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file
       cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory";
        let expected = shlex("cp target/release/fixit .");
        assert_eq!(Some(expected), cp_cwd(cmd, error));
    }

    #[test]
    pub fn non_cp_cmd() {
        let cmd = shlex("git push");
        let error = "";
        assert_eq!(None, cp_cwd(cmd, error));
    }

    #[test]
    pub fn unrelated_error() {
        let cmd = shlex("cp a b");
        let error = "cp: a: no such file or directory";
        assert_eq!(None, cp_cwd(cmd, error));
    }
}
