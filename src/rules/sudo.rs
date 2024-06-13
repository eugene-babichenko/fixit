pub fn sudo(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !error.contains("permission denied") || &cmd[0] == "sudo" {
        return None;
    }

    let mut cmd = cmd.to_vec();
    cmd.insert(0, "sudo".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::sudo;

    #[test]
    fn permission_denied() {
        let cmd = &[
            "mv".to_string(),
            "/etc/pam.d".to_string(),
            "/etc/pam.d.bak".to_string(),
        ];
        let error = "mv: rename /etc/pam.d to /etc/pam.d.bak: permission denied";
        assert_eq!(
            Some(vec![
                "sudo".to_string(),
                "mv".to_string(),
                "/etc/pam.d".to_string(),
                "/etc/pam.d.bak".to_string()
            ]),
            sudo(cmd, error)
        );
    }
}
