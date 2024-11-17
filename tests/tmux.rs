use xshell::cmd;

#[test]
fn fixed() {
    let sh = xshell::Shell::new().unwrap();

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
        "tmux send-keys -t test_session 'FIXIT_LOG=\"fixit::get_text=debug\" fix' C-m"
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

    std::thread::sleep(std::time::Duration::from_millis(1000));

    cmd!(sh, "tmux kill-session -t test_session").run().unwrap();

    println!("{}", res);

    assert!(res.contains("got fast output"));
    assert!(res.contains("Hello, world!"));
}
