pub fn mkdir_missing_parent(cmd: &[String], error: &str) -> Option<Vec<String>> {
    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "mkdir")?;
    if !error.contains("no such file or directory") {
        return None;
    }

    let mut res = cmd.to_vec();
    res.insert(idx + 1, "-p".to_string());
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::mkdir_missing_parent;

    #[test]
    fn mkdir() {
        let cmd = &["mkdir".to_string(), "hello/world".to_string()];
        let error = "no such file or directory";
        assert_eq!(
            Some(vec![
                "mkdir".to_string(),
                "-p".to_string(),
                "hello/world".to_string()
            ]),
            mkdir_missing_parent(cmd, error)
        );
    }
}
