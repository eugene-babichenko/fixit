use std::process::Command;

use crate::rules::utils::find_incorrect_arg;

pub fn fishmarks_bookmark_typo(cmd: Vec<String>, error: &str) -> Vec<Vec<String>> {
    let Some(idx) = find_incorrect_arg(r"'([^']+)'\sbookmark\sdoes\snot\sexist", &cmd, error)
    else {
        return Vec::new();
    };
    let bookmarks = Command::new("fish")
        .args(["-c", "l --names-only"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.lines().map(String::from).collect())
        .unwrap_or_default();
    fishmarks_bookmark_typo_impl(cmd, idx, bookmarks)
}

fn fishmarks_bookmark_typo_impl(
    cmd: Vec<String>,
    idx: usize,
    bookmarks: Vec<String>,
) -> Vec<Vec<String>> {
    bookmarks
        .into_iter()
        .map(|bookmark| {
            let mut fixed = cmd.clone();
            fixed[idx] = bookmark;
            fixed
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shlex::shlex;

    #[test]
    fn suggests_bookmarks() {
        let cmd = shlex("g mrk");
        let bookmarks = vec!["work".to_string(), "dotfiles".to_string()];
        let result = fishmarks_bookmark_typo_impl(cmd, 1, bookmarks);
        assert!(result.contains(&shlex("g work")));
        assert!(result.contains(&shlex("g dotfiles")));
    }

    #[test]
    fn matches_real_error() {
        let cmd = shlex("g mrk");
        let error = "error: 'mrk' bookmark does not exist";
        let Some(idx) = find_incorrect_arg(r"'([^']+)'\sbookmark\sdoes\snot\sexist", &cmd, error)
        else {
            panic!("expected a match");
        };
        assert_eq!(idx, 1);
    }

    #[test]
    fn other_error() {
        let cmd = shlex("g mrk");
        let error = "some unrelated error";
        assert!(fishmarks_bookmark_typo(cmd, error).is_empty());
    }
}
