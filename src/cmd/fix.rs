use std::{
    io,
    time::{Duration, SystemTime},
};

use clap::Parser;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use log::log_enabled;
use rayon::prelude::*;
use strsim::normalized_damerau_levenshtein;
use thiserror::Error;

use crate::{get_text, rules::RULES};

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
    #[error("failed to parse the command")]
    CmdParse,
    #[error("error while rendering the selection menu")]
    Select(#[from] dialoguer::Error),
    #[error(transparent)]
    GetText(#[from] get_text::Error),
}

impl Cmd {
    pub fn run(self) -> Result<(), Error> {
        let bar = ProgressBar::new_spinner().with_message("Getting command output...");
        bar.enable_steady_tick(Duration::from_millis(100));

        let time = SystemTime::now();

        let Some(output) = get_text::get_text(self.get_text, &self.cmd)? else {
            eprintln!("The command ran successfully: nothing to fix.");
            bar.finish_and_clear();
            return Ok(());
        };

        if log_enabled!(log::Level::Debug) {
            let elapsed = SystemTime::now().duration_since(time).unwrap();
            log::debug!("command output in {} milliseconds", elapsed.as_millis());
        }

        bar.set_message("Finding fixes...");

        // split command into parts
        let cmd_split = shlex::split(&self.cmd).ok_or(Error::CmdParse)?;

        let time = SystemTime::now();

        let mut fixes: Vec<_> = RULES
            .par_iter()
            .map(|fixer| {
                output
                    .par_iter()
                    .map(|error| fixer(&cmd_split, &error.to_lowercase()).par_bridge())
                    .flatten()
            })
            .flatten()
            .map(|fixed_cmd| {
                let fixed_cmd = shlex::try_join(fixed_cmd.iter().map(|s| s as &str)).unwrap();
                let similarity = normalized_damerau_levenshtein(&self.cmd, &fixed_cmd);
                log::debug!("fixed command: `{fixed_cmd}`; similarity: {similarity}");
                (fixed_cmd, similarity)
            })
            .collect();

        if log_enabled!(log::Level::Debug) {
            let elapsed = SystemTime::now().duration_since(time).unwrap();
            log::debug!(
                "{} fixes found in {} milliseconds",
                fixes.len(),
                elapsed.as_millis()
            );
        }

        fixes.sort_by(|left, right| right.1.partial_cmp(&left.1).unwrap());
        fixes.dedup_by_key(|v| v.1);
        let fixes: Vec<_> = fixes.into_iter().map(|v| v.0).collect();

        bar.finish_and_clear();

        if fixes.is_empty() {
            eprintln!("No fixes were found!");
            return Ok(());
        }

        // Set empty handler for Ctrl-C. This will cause `Select` to exit with
        // an error instead of immediately interrupting this program. This
        // gives us the possibility to properly show cursor again in a case the
        // user presses Ctrl-C, which is not done automatically by `dialoguer`.
        ctrlc::set_handler(|| {
            Term::stderr()
                .show_cursor()
                .expect("failed to show the cursor again");
        })
        .unwrap();

        let selection_result = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("↓(j)/↑(k)/enter(space)/[q]uit(esc/ctrl-c)")
            .default(0)
            .max_length(self.page_size)
            .items(&fixes)
            .interact_opt();

        // Do not throw an error when Ctrl-C is pressed.
        if let Err(dialoguer::Error::IO(e)) = &selection_result {
            if e.kind() == io::ErrorKind::Interrupted {
                return Ok(());
            }
        }

        if let Some(selection) = selection_result? {
            print!("{}", fixes[selection]);
        }

        Ok(())
    }
}
