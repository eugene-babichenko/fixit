use regex::Regex;

/// Find the name of the incorrect argument in the error message, find
/// that argument in the command and return its index.
pub fn find_incorrect_arg(re: &str, cmd: &[String], error: &str) -> Option<usize> {
    let regex = Regex::new(re).unwrap();
    let arg = regex.captures(error)?.get(1)?.as_str();
    cmd.iter()
        .enumerate()
        .find(|(_, cmd)| *cmd == arg)
        .map(|(arg_idx, _)| arg_idx)
}
