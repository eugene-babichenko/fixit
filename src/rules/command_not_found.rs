use std::{
    env,
    fs::{canonicalize, read_dir},
    path::PathBuf,
};

use regex::Regex;

pub fn command_not_found(cmd: Vec<String>, error: &str) -> Vec<Vec<String>> {
    let Some(path) = env::var_os("PATH") else {
        log::debug!("$PATH not set");
        return Vec::new();
    };
    let path = env::split_paths(&path);
    let shell_items = get_shell_items(env::var("FIXIT_FNS").unwrap_or_else(|_| String::new()));
    command_not_found_impl(cmd, error, path, shell_items)
}

fn get_shell_items(items: String) -> Vec<String> {
    if items.contains(' ') {
        items.split(' ').map(String::from).collect()
    } else {
        items.lines().map(String::from).collect()
    }
}

fn command_not_found_impl(
    cmd: Vec<String>,
    error: &str,
    path: impl Iterator<Item = PathBuf>,
    shell_items: Vec<String>,
) -> Vec<Vec<String>> {
    let Some(idx) = detect_command(&cmd, error) else {
        return Vec::new();
    };

    path.filter_map(|path| read_dir(path).ok())
        .flatten()
        .filter_map(|dir_entry_res| dir_entry_res.ok())
        .filter_map(|dir_entry| canonicalize(dir_entry.path()).ok())
        .filter(|path| path.is_file())
        .filter_map(|path| path.file_name()?.to_str().map(String::from))
        .chain(shell_items)
        .map(|fix| {
            let mut r = cmd.clone();
            r[idx] = fix;
            r
        })
        .collect()
}

fn detect_command(cmd: &[String], error: &str) -> Option<usize> {
    let regex_bash = Regex::new(r"bash:(?: line \d+:)? ([^\s]+): command not found(...)?").unwrap();
    let regex_zsh = Regex::new(r"zsh:(?:\d+:)? command not found: ([^\s]+)").unwrap();
    let regex_fish = Regex::new(r"fish: unknown command: ([^\s]+)").unwrap();
    let regex_powershell = Regex::new(r"the term '([^']+)' is not recognized").unwrap();

    let regex_list = [regex_bash, regex_zsh, regex_fish, regex_powershell];
    let wrong_cmd = regex_list
        .iter()
        .find_map(|regex| regex.captures(error)?.get(1))?;
    cmd.iter()
        .enumerate()
        .find(|(_, c)| c.to_lowercase() == wrong_cmd.as_str())
        .map(|(idx, _)| idx)
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, fs::File, hash::RandomState};

    use rstest::{fixture, rstest};
    use tempfile::{tempdir, TempDir};

    use crate::shlex::shlex;

    use super::*;

    const TEST_CMD: &str = "LOG=info gti status";
    const TEST_CMD_IDX: usize = 1;

    #[rstest]
    #[case("bash: gti: command not found")]
    #[case("bash: gti: command not found...")]
    #[case("zsh: command not found: gti")]
    #[case("fish: unknown command: gti")]
    #[case("gti: the term 'gti' is not recognized as a name of a cmdlet, function, script file, or executable program.")]
    fn detect_command_test(#[case] error: &str) {
        let cmd = shlex(TEST_CMD);
        assert_eq!(Some(TEST_CMD_IDX), detect_command(&cmd, error));
    }

    #[fixture]
    fn path_and_tempdir() -> (Vec<PathBuf>, TempDir, TempDir) {
        let d = tempdir().unwrap();
        File::create(d.path().join("git")).unwrap();
        File::create(d.path().join("tig")).unwrap();
        let d2 = tempdir().unwrap();
        File::create(d2.path().join("lazygit")).unwrap();
        let path = vec![d.path().to_owned(), d2.path().to_owned()];
        env::remove_var("FIXIT_FNS");
        (path, d, d2)
    }

    #[rstest]
    fn test_rule(path_and_tempdir: (Vec<PathBuf>, TempDir, TempDir)) {
        let expected = vec![
            shlex("LOG=info git status"),
            shlex("LOG=info tig status"),
            shlex("LOG=info lazygit status"),
        ];

        let error = "bash: gti: command not found";

        let fixed = command_not_found_impl(
            shlex(TEST_CMD),
            error,
            path_and_tempdir.0.into_iter(),
            Vec::new(),
        );

        let expected: HashSet<_, RandomState> = HashSet::from_iter(expected.into_iter());
        let fixed = HashSet::from_iter(fixed.into_iter());

        assert_eq!(expected, fixed);
    }

    #[rstest]
    fn test_rule_no_match(path_and_tempdir: (Vec<PathBuf>, TempDir, TempDir)) {
        let error = "error: Using `cargo install` to install the binaries from the package in current working directory is no longer supported, use `cargo install --path .` instead. Use `cargo build` if you want to simply build the package.";
        let fixed = command_not_found_impl(
            shlex(TEST_CMD),
            error,
            path_and_tempdir.0.into_iter(),
            Vec::new(),
        );
        assert_eq!(Vec::<Vec<String>>::new(), fixed);
    }

    #[rstest]
    #[case("", vec![])]
    #[case("a b c", vec!["a".to_string(), "b".to_string(), "c".to_string()])]
    #[case("a\nb\nc", vec!["a".to_string(), "b".to_string(), "c".to_string()])]
    fn test_get_shell_items(#[case] env_var: &str, #[case] expected: Vec<String>) {
        let result = get_shell_items(env_var.to_string());
        assert_eq!(expected, result);
    }
}
