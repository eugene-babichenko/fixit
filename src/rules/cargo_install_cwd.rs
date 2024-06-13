const ERROR: &str = "using `cargo install` to install the binaries";

pub fn cargo_install_cwd(cmd: &[String], error: &str) -> Option<Vec<String>> {
    if !error.contains(ERROR) {
        return None;
    }
    let mut res = cmd.to_vec();
    res.extend_from_slice(&["--path".to_string(), ".".to_string()]);
    Some(res)
}
