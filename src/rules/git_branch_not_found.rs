use std::process::Command;

use super::utils::find_incorrect_arg;

pub fn git_branch_not_found(mut cmd: Vec<String>, error: &str) -> Vec<Vec<String>> {
    let Some(n) = find_incorrect_arg(
        r"pathspec '([^']+)' did not match any file\(s\) known to git",
        &cmd,
        error,
    ) else {
        return Vec::new();
    };

    // collect branch names
    let Ok(Ok(branches)) = Command::new("git")
        .args(["for-each-ref", "--format=%(refname:short)", "refs/heads"])
        .output()
        .map(|r| {
            String::from_utf8(r.stdout)
                .map(|s| s.lines().map(|s| s.to_string()).collect::<Vec<_>>())
        })
    else {
        return Vec::new();
    };

    let mut res = Vec::with_capacity(branches.len() + 1);

    for branch in branches {
        let mut cmd = cmd.clone();
        cmd[n] = branch;
        res.push(cmd);
    }

    cmd.insert(n, "-b".to_string());
    res.push(cmd);

    res
}
