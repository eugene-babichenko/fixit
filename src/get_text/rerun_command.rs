use std::{env, process::Command};

use crate::get_text::{stdout_to_string, Error};

pub fn rerun_command(cmd: &str, powershell: bool) -> Result<Option<Vec<String>>, Error> {
    rerun_command_impl(cmd, &get_shell(powershell))
}

fn get_shell(powershell: bool) -> String {
    if powershell {
        return "pwsh".to_string();
    }

    if let Ok(shell) = env::var("SHELL") {
        return shell;
    }

    let shell = if cfg!(target_os = "windows") {
        "cmd.exe".to_string()
    } else {
        "/bin/sh".to_string()
    };

    log::warn!(
        "no $SHELL variable was found, using the default shell: {}",
        shell
    );

    shell
}

fn rerun_command_impl(cmd: &str, shell: &str) -> Result<Option<Vec<String>>, Error> {
    log::debug!("shell in use: {}", &shell);
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

    Ok(Some(vec![stderr, stdout]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let cmd = "echo hello; echo world 1>&2; exit 1";
        assert_eq!(
            Some(vec!["world\n".to_string(), "hello\n".to_string()]),
            rerun_command_impl(cmd, "/bin/sh").unwrap()
        );
    }

    #[test]
    fn command_ran_successfully() {
        let cmd = "echo hello; echo world 1>&2";
        assert_eq!(None, rerun_command_impl(cmd, "/bin/sh").unwrap());
    }
}
