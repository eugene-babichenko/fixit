use std::{
    env,
    fs::{canonicalize, read_dir},
};

use rayon::prelude::*;

pub fn command_not_found(cmd: &[String], error: &str) -> Vec<Vec<String>> {
    if !error.contains("command not found") && !error.contains("unknown command") {
        return Vec::new();
    }

    let Some(path) = env::var_os("PATH") else {
        return Vec::new();
    };
    let path = env::split_paths(&path);

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
            r[0] = f.to_string();
            Some(r)
        })
        .collect()
}
