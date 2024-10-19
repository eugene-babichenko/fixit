pub fn brew_update_upgrade(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !error.contains("this command updates brew itself") {
        return None;
    }

    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "update")?;
    cmd[idx] = "upgrade".to_string();
    Some(cmd)
}

#[cfg(test)]
mod test {
    use crate::shlex::shlex;

    use super::*;

    #[test]
    fn brew_update() {
        let cmd = shlex("brew update git");
        let error = "error: this command updates brew itself, and does not take formula names.";
        let expected = shlex("brew upgrade git");
        assert_eq!(Some(expected), brew_update_upgrade(cmd, error));
    }

    #[test]
    fn other_error() {
        let cmd = shlex("cp ./target/debug/fixit");
        let error = "usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file
    cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory";
        assert_eq!(None, brew_update_upgrade(cmd, error));
    }
}
