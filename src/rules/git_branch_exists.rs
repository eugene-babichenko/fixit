use regex::Regex;

pub fn git_branch_exists(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    let r = Regex::new(r"a branch named '[^']+' already exists").unwrap();
    if !r.is_match(error) {
        return None;
    }
    let n = cmd
        .iter()
        .enumerate()
        .find(|(_, s)| *s == "-b")
        .map(|(i, _)| i)?;
    cmd.remove(n);
    Some(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shlex::shlex;

    #[test]
    fn branch_exists() {
        let cmd = shlex("git checkout -b develop");
        let error = "fatal: a branch named 'develop' already exists";
        let expected = shlex("git checkout develop");
        assert_eq!(Some(expected), git_branch_exists(cmd, error))
    }
}
