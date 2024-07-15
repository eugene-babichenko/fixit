pub fn sudo(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if &cmd[0] != "sudo"
        && !(error.contains("permission denied") || error.contains("operation not permitted"))
    {
        return None;
    }

    cmd.insert(0, "sudo".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::sudo;

    #[test]
    fn permission_denied() {
        let cmd = shlex("mv /etc/pam.d /etc/pam.d.bak");
        let error = "mv: rename /etc/pam.d to /etc/pam.d.bak: permission denied";
        assert_eq!(
            Some(shlex("sudo mv /etc/pam.d /etc/pam.d.bak")),
            sudo(cmd, error)
        );
    }

    #[test]
    fn operation_not_permitted() {
        let cmd = shlex("cp ./target/debug/fixit /bin");
        let error = "cp: /bin/fixit: operation not permitted";
        assert_eq!(
            Some(shlex("sudo cp ./target/debug/fixit /bin")),
            sudo(cmd, error)
        );
    }

    #[test]
    fn other_error() {
        let cmd = shlex("cp ./target/debug/fixit");
        let error = "usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file
    cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory";
        assert_eq!(None, sudo(cmd, error));
    }
}
