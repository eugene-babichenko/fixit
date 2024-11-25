use std::{io, time::SystemTime};

use clap::Parser;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use thiserror::Error;

use crate::{
    get_text,
    rules::{find_fixes, RULES},
};

#[derive(Parser)]
/// Fix a failing command. This command is not meant for direct use by the
/// user. Upon selection it prints out the selected fix to stdout. It fails if
/// no fixes were found.
pub struct Cmd {
    /// The command to fix.
    cmd: String,
    /// The maximum number of fixes to show on the screen.
    #[arg(env = "FIXIT_PAGE_SIZE", default_value_t = 5)]
    page_size: usize,
    #[command(flatten)]
    get_text: crate::get_text::Config,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("error while rendering the selection menu")]
    Select(#[from] dialoguer::Error),
    #[error(transparent)]
    GetText(#[from] get_text::Error),
}

impl Cmd {
    pub fn run(self) -> Result<(), Error> {
        let time = SystemTime::now();

        if self.cmd.is_empty() {
            eprintln!("No previous commands.");
            return Ok(());
        }

        let Some(output) = get_text::get_text(self.get_text, &self.cmd)? else {
            eprintln!("The command ran successfully: nothing to fix.");
            return Ok(());
        };

        let elapsed = SystemTime::now().duration_since(time).unwrap();
        log::debug!("command output in {} milliseconds", elapsed.as_millis());

        let fixes = find_fixes(&self.cmd, output, RULES);

        let elapsed = SystemTime::now().duration_since(time).unwrap();
        log::debug!(
            "{} fixes found in {} milliseconds",
            fixes.len(),
            elapsed.as_millis()
        );

        if fixes.is_empty() {
            eprintln!("No fixes were found!");
            return Ok(());
        }

        // Set empty handler for Ctrl-C. This will cause `Select` to exit with
        // an error instead of immediately interrupting this program.
        ctrlc::set_handler(|| {}).unwrap();

        let selection_result = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("↓(j)/↑(k)/enter(space)/[q]uit(esc/ctrl-c)")
            .default(0)
            .max_length(self.page_size)
            .items(&fixes)
            .interact_opt();

        match selection_result {
            Ok(Some(selection)) => {
                print!("{}", fixes[selection]);
                return Ok(());
            }
            Ok(None) => {}
            // Do not throw an error when Ctrl-C is pressed.
            Err(dialoguer::Error::IO(e)) if e.kind() == io::ErrorKind::Interrupted => {
                Term::stderr()
                    .show_cursor()
                    .expect("failed to show the cursor again");
            }
            Err(e) => return Err(Error::Select(e)),
        }

        eprintln!("Cancelled.");
        Ok(())
    }
}
