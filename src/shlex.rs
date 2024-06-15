pub fn shlex(cmd: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut word = String::new();
    let mut quote_state = None;
    let mut escape_state = false;

    for c in cmd.chars() {
        if !escape_state && c == '\\' {
            escape_state = true;
            word.push(c);
            continue;
        }

        if escape_state {
            escape_state = false;
            word.push(c);
            continue;
        }

        if quote_state.is_none() && (c == '"' || c == '\'') {
            quote_state = Some(c);
            word.push(c);
            continue;
        }

        if let Some(q) = quote_state {
            if c == q {
                quote_state = None;
            }
            word.push(c);
            continue;
        }

        if c.is_whitespace() {
            if !word.is_empty() {
                res.push(word);
                word = String::new();
            }
            continue;
        }

        word.push(c)
    }

    if !word.is_empty() {
        res.push(word);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::shlex;

    #[test]
    fn simple() {
        let cmd = "git push origin master";
        let expected = vec![
            "git".to_string(),
            "push".to_string(),
            "origin".to_string(),
            "master".to_string(),
        ];
        assert_eq!(expected, shlex(cmd));
    }

    #[test]
    fn with_quotes() {
        let cmd = "git commit -m \"initial commit\" --amend";
        let expected = vec![
            "git".to_string(),
            "commit".to_string(),
            "-m".to_string(),
            "\"initial commit\"".to_string(),
            "--amend".to_string(),
        ];
        assert_eq!(expected, shlex(cmd));
    }

    #[test]
    fn escaped_chars_inside_quotes() {
        let cmd = "git commit -m \"test \\\"escaping\\\"\"";
        let expected = vec![
            "git".to_string(),
            "commit".to_string(),
            "-m".to_string(),
            "\"test \\\"escaping\\\"\"".to_string(),
        ];
        assert_eq!(expected, shlex(cmd));
    }
}
