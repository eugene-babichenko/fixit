use std::{env, process::Command};

use crate::get_text::{stdout_to_string, Error};

pub fn rerun_command(cmd: &str) -> Result<Option<(String, String)>, Error> {
    // re-run the command in the current shell
    let shell = env::var("SHELL")?;

    log::debug!("re-running command: {}", &cmd);
    let output = Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .output()
        .map_err(Error::ReRun)?;

    // if the command is successful we have nothing to do
    if output.status.success() {
        return Ok(None);
    }

    let stderr = stdout_to_string(output.stderr)?;
    let stdout = stdout_to_string(output.stdout)?;

    log::debug!("command stderr: {}", stderr);
    log::debug!("command stdout: {}", stdout);

    Ok(Some((stderr, stdout)))
}
