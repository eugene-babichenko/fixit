pub fn cp_dir(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !error.contains("cp") || !error.contains("is a directory") {
        return None;
    }

    let mut cmd = cmd.to_vec();
    let (idx, _) = cmd.iter().enumerate().find(|(_, s)| *s == "cp")?;
    cmd.insert(idx + 1, "-R".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::cp_dir;

    #[test]
    fn cp_dir_test() {
        let cmd = &["cp".to_string(), "src/".to_string(), "target/".to_string()];
        let error = "cp: src/ is a directory (not copied).";
        let expected = vec![
            "cp".to_string(),
            "-R".to_string(),
            "src/".to_string(),
            "target/".to_string(),
        ];
        assert_eq!(expected, cp_dir(cmd, error).unwrap());
    }
}
