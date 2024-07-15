const ERROR: &str = "using `cargo install` to install the binaries";

pub fn cargo_install_cwd(mut cmd: Vec<String>, error: &str) -> Option<Vec<String>> {
    if !error.contains(ERROR) {
        return None;
    }
    cmd.extend_from_slice(&["--path".to_string(), ".".to_string()]);
    Some(cmd)
}

#[cfg(test)]
mod test {
    use crate::shlex::shlex;

    use super::*;

    #[test]
    fn cargo_install_cwd_test() {
        let cmd = shlex("cargo install");
        let error = "error: Using `cargo install` to install the binaries from the package in current working directory is no longer supported, use `cargo install --path .` instead. Use `cargo build` if you want to simply build the package.";
        let expected = shlex("cargo install --path .");
        assert_eq!(
            Some(expected),
            cargo_install_cwd(cmd, &error.to_lowercase())
        );
    }
}
