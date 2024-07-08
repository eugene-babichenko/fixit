# Contributing guidelines

## Setting up development environment

The quickest way to start contributing is to use GitHub Codespaces (see
README.md).

You can just use a devcontainer included with this project.

If you do not want to use the devcontainer, you will need the following
dependencies:

- The Rust toolchain.
- Prettier for markdown formatting.
- lefthook for git pre-commit hooks (optional but advised).

## Markdown formatting

All markdown files should be formatted with:

    prettier --prose-wrap always

## Adding new rules

A rule is a function that attempts to fix a command. To add a rule named
`rule_name` you need to add a file named `src/rules/rule_name.rs`. This file
should contain a `pub` function named `rule_name` with the following signature:

```rust
/// # Arguments
///
/// - `cmd` - tokenized command, e.g. (["git", "commit", "-m", "initial commit"])
/// - `error` - the output of the command.
fn rule_name(cmd: &[String], error: &str) -> T
```

Where `T` is any type implementing `IntoIterator<Item = Vec<String>>`. Normally,
this would be `Option<Vec<String>>` or `Vec<Vec<String>>`. `Vec<String>`
contains a command, e.g. `["git", "commit", "--amend"]`.

Then the name should be added to `src/rules/mod.rs` as an argument to the
`define_rules` macro.

## Adding shell support

Shells support should be added to `src/cmd/init.rs`. The code here is pretty
self-explanatory.

A couple notes on variables that go into generating the fix function:

- `executable` - the name of `fixit` executable.
- `name` - the name of the generated function.

All shell support functions should generally follow this pseudocode:

```
function {name}
    last_command = get_last_command()
    fixed_command = {executable} fix $last_command
    if $fixed_command != ""
        run($fixed_command)
        add_to_history($fixed_command)
    end
end
```

## Adding support for terminal emulators/multiplexers

See `src/get_text`. There is no specific guideline for this.
