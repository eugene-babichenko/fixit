pub fn cp_dir(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !error.contains("cp") || !error.contains("is a directory") {
        return None;
    }

    let (idx, _) = cmd.iter().enumerate().find(|(_, s)| *s == "cp")?;
    cmd.insert(idx + 1, "-R".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::cp_dir;

    #[test]
    fn cp_dir_test() {
        let cmd = shlex("cp src/ target/");
        let error = "cp: src/ is a directory (not copied).";
        let expected = shlex("cp -R src/ target/");
        assert_eq!(expected, cp_dir(cmd, error).unwrap());
    }
}
