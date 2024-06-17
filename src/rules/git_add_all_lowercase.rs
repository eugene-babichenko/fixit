pub fn git_add_all_lowercase(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !cmd.contains(&"git".to_string())
        || !cmd.contains(&"add".to_string())
        || !error.contains(&"unknown switch".to_string())
    {
        return None;
    }

    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "-a")?;
    let mut cmd = cmd.to_vec();
    cmd[idx] = "-A".to_string();
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::git_add_all_lowercase;

    #[test]
    fn git_add_all_lowercase_test() {
        let cmd = &["git".to_string(), "add".to_string(), "-a".to_string()];
        let error = "error: unknown switch `a'";
        let expected = vec!["git".to_string(), "add".to_string(), "-A".to_string()];
        assert_eq!(expected, git_add_all_lowercase(cmd, error).unwrap());
    }
}
