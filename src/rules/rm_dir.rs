pub fn rm_dir(cmd: &[String], error: &str) -> Option<Vec<String>> {
    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "rm")?;
    if !error.contains("is a directory") {
        return None;
    }

    let mut cmd = cmd.to_vec();
    cmd.insert(idx + 1, "-r".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    #[test]
    fn rm_dir_test() {
        let cmd = &["rm".to_string(), "src".to_string()];
        let error = "rm: src: is a directory";
        let expected = vec!["rm".to_string(), "-r".to_string(), "src".to_string()];
        assert_eq!(expected, super::rm_dir(cmd, error).unwrap());
    }
}
