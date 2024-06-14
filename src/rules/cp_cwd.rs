pub fn cp_cwd(cmd: &[String], _error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"cd".to_string()) || !cmd.len() == 2 {
        return None;
    }

    let mut cmd = cmd.to_vec();
    cmd.push(".".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::cp_cwd;

    #[test]
    pub fn non_cp_cmd() {
        let cmd = &["git".to_string(), "push".to_string()];
        let error = "";
        assert_eq!(None, cp_cwd(cmd, error));
    }
}
