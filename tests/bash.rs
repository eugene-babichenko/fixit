use std::time::Duration;
use std::{env, process::Command};

use expectrl::Session;
use tempfile::NamedTempFile;

#[test]
fn bash() {
    let histfile = NamedTempFile::new().unwrap();

    let mut bash = Command::new("bash");
    bash.args(["--norc", "-i", "-o", "history"])
        .env(
            "PATH",
            &format!("./target/debug/:{}", env::var("PATH").unwrap()),
        )
        .env("FIXIT_QUICK_ENABLE", "false")
        .env("HISTFILE", histfile.path())
        .env("SHELL", "/bin/bash");

    let mut p = Session::spawn(bash).expect("Failed to spawn bash");

    p.set_expect_timeout(Some(Duration::from_secs(1)));

    p.send_line("eval \"$(fixit init bash)\"").unwrap();
    p.send_line("eco 'Hello, world!'").unwrap();
    p.expect("command not found").unwrap();
    p.send_line("fix").unwrap();
    p.send_line("").unwrap();
    p.expect("Hello, world!").unwrap();
    p.send_line("exit").unwrap();
}
