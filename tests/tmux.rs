use std::{env, io::Read, process::Command, time::Duration};

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
        .env("HISTFILE", histfile.path())
        .env("TERM", "xterm-256color");

    let mut p = Session::spawn(tmux).expect("Failed to spawn tmux");

    p.send_line("eval \"$(fixit init bash)\"").unwrap();

    p.set_expect_timeout(Some(Duration::from_secs(5)));

    p.send_line("export SHELL=bash").unwrap();
    p.send_line(format!(
        "export PATH='{}/target/debug/:$PATH'",
        env::current_dir().unwrap().display()
    ))
    .unwrap();
    p.send_line("eco 'Hello, world!'").unwrap();
    p.send_line("FIXIT_LOG='fixit::get_text=debug' fix")
        .unwrap();
    p.send_line("").unwrap();
    p.expect("got tmux output").unwrap();
    p.expect("got fast output").unwrap();
    p.expect("Hello, world!").unwrap();
    p.send_line("exit").unwrap();
    // let mut buf = String::new();
    // p.read_to_string(&mut buf).unwrap();
    // println!("{}", buf);
}
