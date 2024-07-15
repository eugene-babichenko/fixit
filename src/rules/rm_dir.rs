pub fn rm_dir(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    let (idx, _) = cmd.iter().enumerate().find(|(_, c)| *c == "rm")?;
    if !error.contains("is a directory") {
        return None;
    }

    cmd.insert(idx + 1, "-r".to_string());
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use crate::shlex::shlex;

    #[test]
    fn rm_dir_test() {
        let cmd = shlex("rm src");
        let error = "rm: src: is a directory";
        let expected = shlex("rm -r src");
        assert_eq!(expected, super::rm_dir(cmd, error).unwrap());
    }
}
