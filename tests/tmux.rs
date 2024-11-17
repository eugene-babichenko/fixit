use std::{env, process::Command, thread::sleep, time::Duration};

use expectrl::Session;
use tempfile::NamedTempFile;

#[test]
fn fixed() {
    let histfile = NamedTempFile::new().unwrap();

    let mut tmux = Command::new("tmux");
    tmux.args(["new-session", "bash", "--norc", "-i", "-o", "history"])
        .env(
            "PATH",
            &format!(
                "{}/target/debug/:{}",
                env::current_dir().unwrap().display(),
                env::var("PATH").unwrap()
            ),
        )
        .env("HISTFILE", histfile.path());

    let mut p = Session::spawn(tmux).expect("Failed to spawn tmux");

    p.send_line("eval \"$(fixit init bash)\"").unwrap();
    p.set_expect_timeout(Some(Duration::from_secs(5)));

    p.send_line("export SHELL=bash").unwrap();
    p.send_line("eco 'Hello, world!'").unwrap();
    p.send_line("FIXIT_LOG='fixit::get_text=debug' fix")
        .unwrap();
    p.send_line("").unwrap();
    sleep(Duration::from_secs(1));
    p.expect("got tmux output").unwrap();
    p.expect("got fast output").unwrap();
    p.expect("Hello, world!").unwrap();
    p.send_line("exit").unwrap();
}
