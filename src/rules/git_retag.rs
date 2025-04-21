use regex::Regex;

pub fn git_retag(cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    let regex = Regex::new(r#"tag\s'([^']+)'\salready\sexists"#).unwrap();
    let tag = regex.captures(error)?.get(1)?.as_str().to_string();
    Some(
        [
            vec![
                "git".to_string(),
                "tag".to_string(),
                "-d".to_string(),
                tag,
                "&&".to_string(),
            ],
            cmd,
        ]
        .concat(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shlex::shlex;

    #[test]
    fn git_tag_exists() {
        let cmd = shlex("git tag v0.10.0");
        let error = "fatal: tag 'v0.10.0' already exists";
        let expected = shlex("git tag -d v0.10.0 && git tag v0.10.0");
        assert_eq!(Some(expected), git_retag(cmd, error));
    }
}
