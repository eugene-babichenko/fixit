use xshell::{cmd, Shell};

/// Kills the tmux test session on scope exit, including on panic, so a
/// failed assertion or unwrap doesn't leave the session (and whatever is
/// still running inside it, e.g. `fixit` blocked on interactive input)
/// orphaned on the machine.
struct KillSessionOnDrop<'a>(&'a Shell);

impl Drop for KillSessionOnDrop<'_> {
    fn drop(&mut self) {
        let sh = self.0;
        let _ = cmd!(sh, "tmux kill-session -t test_session").run();
    }
}

#[test]
fn fixed() {
    let sh = Shell::new().unwrap();
    let _guard = KillSessionOnDrop(&sh);

    cmd!(
        sh,
        "tmux new-session -d -s test_session bash --norc -i -o history"
    )
    .run()
    .unwrap();

    cmd!(sh, "tmux send-keys -t test_session 'export SHELL=bash' C-m")
        .run()
        .unwrap();

    cmd!(
        sh,
        "tmux send-keys -t test_session 'export PATH=\"$PWD/target/debug/:$PATH\"' C-m"
    )
    .run()
    .unwrap();

    cmd!(
        sh,
        "tmux send-keys -t test_session 'eval \"$(fixit init bash)\"' C-m"
    )
    .run()
    .unwrap();

    cmd!(
        sh,
        "tmux send-keys -t test_session 'eco \"Hello, world!\"' C-m"
    )
    .run()
    .unwrap();

    cmd!(
        sh,
        "tmux send-keys -t test_session 'RUST_LOG=\"fixit::get_text=debug\" fix' C-m"
    )
    .run()
    .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));

    cmd!(sh, "tmux send-keys -t test_session Enter")
        .run()
        .unwrap();

    let res = cmd!(sh, "tmux capture-pane -t test_session -p")
        .read()
        .unwrap();

    println!("{res}");

    assert!(res.contains("got fast output"));
    assert!(res.contains("Hello, world!"));
}
