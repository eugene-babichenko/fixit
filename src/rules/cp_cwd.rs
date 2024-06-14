pub fn cp_cwd(cmd: &[String], _error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"cd".to_string()) && !cmd.len() == 2 {
        return None;
    }

    let mut cmd = cmd.to_vec();
    cmd.push(".".to_string());
    Some(cmd)
}
