use std::{
    env,
    fs::{canonicalize, read_dir},
    path::PathBuf,
};

use rayon::prelude::*;
use regex::Regex;

pub fn command_not_found(cmd: Vec<String>, error: &str) -> Vec<Vec<String>> {
    let Some(path) = env::var_os("PATH") else {
        log::debug!("$PATH not set");
        return Vec::new();
    };
    let path = env::split_paths(&path);
    command_not_found_impl(cmd, error, path)
}

fn command_not_found_impl(
    cmd: Vec<String>,
    error: &str,
    path: impl Iterator<Item = PathBuf> + Send,
) -> Vec<Vec<String>> {
    let Some(idx) = detect_command(&cmd, error) else {
        return Vec::new();
    };

    path.par_bridge()
        .filter_map(|path| read_dir(path).map(|res| res.par_bridge()).ok())
        .flatten()
        .filter_map(|dir_entry_res| dir_entry_res.ok())
        .filter_map(|dir_entry| canonicalize(dir_entry.path()).ok())
        .filter(|path| path.is_file())
        .filter_map(|path| {
            let f = path.file_name()?;
            let f = f.to_str()?;
            let mut r = cmd.clone();
            r[idx] = f.to_string();
            Some(r)
        })
        .collect()
}

fn detect_command(cmd: &[String], error: &str) -> Option<usize> {
    let regex_bash = Regex::new(r"bash: ([^\s]+): command not found").unwrap();
    let regex_zsh = Regex::new(r"zsh: command not found: ([^\s]+)").unwrap();
    let regex_fish = Regex::new(r"fish: unknown command: ([^\s]+)").unwrap();

    let regex_list = [regex_bash, regex_zsh, regex_fish];
    let wrong_cmd = regex_list
        .iter()
        .find_map(|regex| regex.captures(error)?.get(1))?;
    cmd.iter()
        .enumerate()
        .find(|(_, c)| *c == wrong_cmd.as_str())
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
    #[case("zsh: command not found: gti")]
    #[case("fish: unknown command: gti")]
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

        let fixed = command_not_found_impl(shlex(TEST_CMD), error, path_and_tempdir.0.into_iter());

        let expected: HashSet<_, RandomState> = HashSet::from_iter(expected.into_iter());
        let fixed = HashSet::from_iter(fixed.into_iter());

        assert_eq!(expected, fixed);
    }

    #[rstest]
    fn test_rule_no_match(path_and_tempdir: (Vec<PathBuf>, TempDir, TempDir)) {
        let error = "error: Using `cargo install` to install the binaries from the package in current working directory is no longer supported, use `cargo install --path .` instead. Use `cargo build` if you want to simply build the package.";
        let fixed = command_not_found_impl(shlex(TEST_CMD), error, path_and_tempdir.0.into_iter());
        assert_eq!(Vec::<Vec<String>>::new(), fixed);
    }
}
