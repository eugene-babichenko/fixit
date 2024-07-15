pub fn sudo(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !error.contains("permission denied") || &cmd[0] == "sudo" {
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
}
