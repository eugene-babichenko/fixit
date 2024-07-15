pub fn mkdir_missing_parent(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "mkdir")?;
    if !error.contains("no such file or directory") {
        return None;
    }

    cmd.insert(idx + 1, "-p".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    use super::mkdir_missing_parent;

    #[test]
    fn mkdir() {
        let cmd = shlex("mkdir hello/world");
        let error = "no such file or directory";
        assert_eq!(
            Some(shlex("mkdir -p hello/world")),
            mkdir_missing_parent(cmd, error)
        );
    }
}
