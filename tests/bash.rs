use std::time::Duration;
use std::{env, process::Command};

use expectrl::Session;
use rstest::{fixture, rstest};
use tempfile::NamedTempFile;

#[fixture]
fn bash() -> (Session, NamedTempFile) {
    let histfile = NamedTempFile::new().unwrap();

    let mut bash = Command::new("bash");
    bash.args(["--norc", "-i", "-o", "history"])
        .env(
            "PATH",
            &format!(
                "{}/target/debug/:{}",
                env::current_dir().unwrap().display(),
                env::var("PATH").unwrap()
            ),
        )
        .env("FIXIT_QUICK_ENABLE", "false")
        .env("HISTFILE", histfile.path())
        .env("SHELL", "bash");

    let mut p = Session::spawn(bash).expect("Failed to spawn bash");

    p.send_line("eval \"$(fixit init bash)\"").unwrap();

    p.set_expect_timeout(Some(Duration::from_secs(5)));

    (p, histfile)
}

#[rstest]
fn fixed(bash: (Session, NamedTempFile)) {
    let (mut p, _histfile) = bash;
    p.send_line("eco 'Hello, world!'").unwrap();
    p.expect("command not found").unwrap();
    p.send_line("fix").unwrap();
    p.send_line("").unwrap();
    p.expect("Hello, world!").unwrap();
    p.send_line("exit").unwrap();
}

#[rstest]
fn nothing_to_fix(bash: (Session, NamedTempFile)) {
    let (mut p, _histfile) = bash;
    p.send_line("echo 'Hello, world!'").unwrap();
    p.send_line("fix").unwrap();
    p.send_line("").unwrap();
    p.expect("nothing to fix").unwrap();
}

#[rstest]
fn quit(bash: (Session, NamedTempFile)) {
    let (mut p, _histfile) = bash;
    p.send_line("eco 'Hello, world!'").unwrap();
    p.expect("command not found").unwrap();
    p.send_line("fix").unwrap();
    p.send_line("q").unwrap();
    p.expect("Cancelled.").unwrap();
}
