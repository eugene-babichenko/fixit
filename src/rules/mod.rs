use strsim::normalized_damerau_levenshtein;

use crate::shlex::shlex;

/// Process command and its results to return a possible correct command. The
/// result doesn't necesserily have to be a perfect one, it is _just a
/// possibility_. In fact, you can be optimistic with what you return.
///
/// # Args
///
/// - `cmd` - a tokenized commands (the command name and its arguments).
/// - `error` - `stderr` of that command. Note that it comes with all letters
///   lowercased.
///
/// # Returns
///
/// An iterator of possible substitutions as tokenized commands. Empty list
/// means no possible substitutions were found by this fixer.
pub type Rule = fn(cmd: Vec<String>, error: &str) -> Box<dyn Iterator<Item = Vec<String>> + Send>;

macro_rules! wrap_rule {
    ($name:ident) => {
        |cmd: Vec<String>, error: &str| -> Box<dyn Iterator<Item = Vec<String>> + Send> {
            Box::new($name::$name(cmd, error).into_iter())
        }
    };
}

macro_rules! define_rules {
    ($($name:ident),+,) => {
        // Import all modules;
        $(mod $name;)+

        // Define the list of fixers. The name of a fixer is expected to be the
        // same as the name of the module that contains it.
        pub const RULES: &[Rule] = &[
            $(wrap_rule!($name)),+
        ];
    };
}

// Please keep these in the alphanumeric order.
define_rules!(
    brew_update_upgrade,
    cargo_clippy_args,
    cargo_install_cwd,
    cargo_wrong_command,
    command_not_found,
    cp_cwd,
    cp_dir,
    git_add_all_lowercase,
    git_commit_no_changes,
    git_commit_untracked_files,
    git_no_upstream,
    git_wrong_command,
    mkdir_missing_parent,
    rm_dir,
    sudo,
);

pub fn find_fixes(cmd: &str, output: Vec<String>, rules: &[Rule]) -> Vec<String> {
    // split command into parts
    let cmd_split = shlex(cmd);

    let mut fixes: Vec<_> = rules
        .iter()
        .flat_map(|fixer| {
            output
                .iter()
                .flat_map(|error| fixer(cmd_split.clone(), &error.to_lowercase()))
        })
        .map(|fixed_cmd| {
            let fixed_cmd = fixed_cmd.join(" ");
            let similarity = normalized_damerau_levenshtein(cmd, &fixed_cmd);
            log::debug!("fixed command: `{fixed_cmd}`; similarity: {similarity}");
            (fixed_cmd, similarity)
        })
        .collect();

    fixes.sort_by(|(_, left), (_, right)| right.partial_cmp(left).unwrap());
    fixes.dedup_by_key(|(fix, _)| fix.clone());
    fixes.into_iter().map(|(fix, _)| fix).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate() {
        fn r(_cmd: Vec<String>, _error: &str) -> Box<dyn Iterator<Item = Vec<String>> + Send> {
            Box::new(Some(vec!["git".to_string()]).into_iter())
        }
        let rules: &[Rule] = &[r, r];
        assert_eq!(vec!["git"], find_fixes("", vec!["".to_string()], rules));
    }

    #[test]
    fn sorting() {
        fn r1(_cmd: Vec<String>, _error: &str) -> Box<dyn Iterator<Item = Vec<String>> + Send> {
            Box::new(Some(vec!["git".to_string()]).into_iter())
        }
        fn r2(_cmd: Vec<String>, _error: &str) -> Box<dyn Iterator<Item = Vec<String>> + Send> {
            Box::new(Some(vec!["qwerty".to_string()]).into_iter())
        }
        let rules: &[Rule] = &[r2, r1];
        assert_eq!(
            vec!["git".to_string(), "qwerty".to_string()],
            find_fixes("gti", vec!["".to_string()], rules)
        );
    }
}
