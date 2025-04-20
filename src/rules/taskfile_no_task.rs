use regex::Regex;

use crate::rules::utils::find_incorrect_arg;

pub fn taskfile_no_task(cmd: Vec<String>, error: &str) -> Vec<Vec<String>> {
    let old_task_idx = match find_incorrect_arg(r#"task\s"([^"]+)"\sdoes\snot\sexist"#, &cmd, error)
    {
        Some(v) => v,
        None => return Vec::new(),
    };

    let line_regex = Regex::new(r"\*\s([^\s:]+)").unwrap();
    error
        .lines()
        .filter_map(|l| line_regex.captures(l)?.get(1))
        .map(|l| {
            let mut cmd = cmd.clone();
            cmd[old_task_idx] = l.as_str().to_string();
            cmd
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::taskfile_no_task;
    use crate::shlex::shlex;

    #[test]
    fn task_does_not_exist() {
        let cmd = shlex("task lin");
        let err = r#"task: Available tasks for this project:
* lint:     Run linters
* build:    Build project
task: Task "lin" does not exist"#;
        let expected = vec![shlex("task lint"), shlex("task build")];
        assert_eq!(expected, taskfile_no_task(cmd, &err.to_lowercase()));
    }
}
