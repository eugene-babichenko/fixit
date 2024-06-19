use std::{
    env,
    fs::{canonicalize, read_dir},
    path::PathBuf,
};

use rayon::prelude::*;
use regex::Regex;

pub fn command_not_found(cmd: &[String], error: &str) -> Vec<Vec<String>> {
    let Some(path) = env::var_os("PATH") else {
        log::debug!("$PATH not set");
        return Vec::new();
    };
    let path = env::split_paths(&path);

    command_not_found_impl(cmd, error, path)
}

fn command_not_found_impl(
    cmd: &[String],
    error: &str,
    path: impl Iterator<Item = PathBuf> + Send,
) -> Vec<Vec<String>> {
    let Some(idx) = detect_command(cmd, error) else {
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
            let mut r = cmd.to_vec();
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
    let wrong_cmd = regex_list.iter().find_map(|regex| {
        regex
            .captures_iter(error)
            .map(|c| c.extract::<1>().1[0])
            .next()
    })?;
    cmd.iter()
        .enumerate()
        .find(|(_, c)| *c == wrong_cmd)
        .map(|(idx, _)| idx)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

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
}
