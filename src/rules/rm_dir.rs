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

    #[test]
    fn other_error() {
        let cmd = shlex("cp ./target/debug/fixit");
        let error = "usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file
    cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory";
        assert_eq!(None, super::rm_dir(cmd, error));
    }

    #[test]
    fn other_rm_error() {
        let cmd = shlex("rm test");
        let error = "rm: test: no such file or directory";
        assert_eq!(None, super::rm_dir(cmd, error));
    }
}
