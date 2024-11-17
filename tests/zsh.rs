use std::{env, process::Command, time::Duration};

use expectrl::Session;
use rstest::{fixture, rstest};
use tempfile::NamedTempFile;

#[fixture]
fn zsh() -> (Session, NamedTempFile) {
    let histfile = NamedTempFile::new().unwrap();

    let mut zsh = Command::new("zsh");
    zsh.args(["-f", "-i"])
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
        .env("SHELL", "zsh");

    let mut p = Session::spawn(zsh).expect("Failed to spawn zsh");

    p.send_line("eval \"$(fixit init zsh)\"").unwrap();

    (p, histfile)
}

#[rstest]
fn fixed(zsh: (Session, NamedTempFile)) {
    let (mut p, _histfile) = zsh;

    p.set_expect_timeout(Some(Duration::from_secs(5)));

    p.send_line("eco 'Hello, world!'").unwrap();
    p.expect("command not found").unwrap();
    p.send_line("fix").unwrap();
    p.send_line("").unwrap();
    p.expect("Hello, world!").unwrap();
    p.send_line("exit").unwrap();
}
