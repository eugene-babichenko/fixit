use std::process::Command;

use crate::rules::utils::find_incorrect_arg;

pub fn git_branch_typo(cmd: Vec<String>, error: &str) -> Vec<Vec<String>> {
    let Some(idx) = find_incorrect_arg(r"pathspec\s'([^']+)'\sdid\snot\smatch", &cmd, error)
    else {
        return Vec::new();
    };
    let branches = Command::new("git")
        .args(["branch", "--list", "--format=%(refname:short)"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.lines().map(String::from).collect())
        .unwrap_or_default();
    git_branch_typo_impl(cmd, idx, branches)
}

fn git_branch_typo_impl(cmd: Vec<String>, idx: usize, branches: Vec<String>) -> Vec<Vec<String>> {
    branches
        .into_iter()
        .map(|branch| {
            let mut fixed = cmd.clone();
            fixed[idx] = branch;
            fixed
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shlex::shlex;

    #[test]
    fn suggests_local_branches() {
        let cmd = shlex("git checkout masttr");
        let branches = vec!["master".to_string(), "develop".to_string()];
        let result = git_branch_typo_impl(cmd, 2, branches);
        assert!(result.contains(&shlex("git checkout master")));
        assert!(result.contains(&shlex("git checkout develop")));
    }

    #[test]
    fn other_error() {
        let cmd = shlex("git checkout -j hello");
        let error = "error: unknown switch `j'";
        assert!(git_branch_typo(cmd, error).is_empty());
    }

    #[test]
    fn suggests_master_in_this_repo() {
        let cmd = shlex("git checkout masttr");
        let error = "error: pathspec 'masttr' did not match any file(s) known to git";
        let result = git_branch_typo(cmd, error);
        assert!(result.contains(&shlex("git checkout master")));
    }
}
