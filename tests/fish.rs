use std::{env, process::Command, time::Duration};

use expectrl::Session;
use tempfile::NamedTempFile;

#[test]
fn fixed() {
    let histfile = NamedTempFile::new().unwrap();

    let mut fish = Command::new("fish");
    fish.args(["--no-config", "--interactive", "--private"])
        .env(
            "PATH",
            &format!(
                "{}/target/debug/:{}",
                env::current_dir().unwrap().display(),
                env::var("PATH").unwrap()
            ),
        )
        .env("FIXIT_QUICK_ENABLE", "false")
        .env("fish_history", histfile.path())
        .env("SHELL", "fish");

    let mut p = Session::spawn(fish).expect("Failed to spawn fish");

    p.send_line("fixit init fish | source").unwrap();

    p.set_expect_timeout(Some(Duration::from_secs(5)));

    p.send_line("eco 'Hello, world!'").unwrap();
    p.send_line("fix").unwrap();
    p.send_line("").unwrap();
    p.expect("Hello, world!").unwrap();
    p.send_line("exit").unwrap();
}
