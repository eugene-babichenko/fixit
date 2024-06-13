pub fn brew_update_upgrade(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !error.contains("this command updates brew itself") {
        return None;
    }

    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "update")?;
    let mut cmd = cmd.to_vec();
    cmd[idx] = "upgrade".to_string();
    Some(cmd)
}
