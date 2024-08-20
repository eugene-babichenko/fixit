use std::{env, fs::File, io::Read, process::Command};

use tempfile::NamedTempFile;

use super::find_command_output;

pub fn get_text(cmd: &str, depth: usize) -> Option<String> {
    env::var("ZELLIJ").ok()?;
    let f = NamedTempFile::new().ok()?;
    Command::new("zellij")
        .args(["action", "dump-screen", "--full", f.path().to_str()?])
        .output()
        .ok()?;
    let mut file = File::open(f.path()).ok()?;
    let mut output = Vec::new();
    file.read_to_end(&mut output).ok()?;
    find_command_output(cmd, output, depth)
}
