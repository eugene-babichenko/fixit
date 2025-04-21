# `fixit`

![GitHub Release](https://img.shields.io/github/v/release/eugene-babichenko/fixit)
[![Build status](https://github.com/eugene-babichenko/fixit/actions/workflows/tests.yml/badge.svg)](https://github.com/eugene-babichenko/fixit/actions)
[![Coverage Status](https://coveralls.io/repos/github/eugene-babichenko/fixit/badge.svg)](https://coveralls.io/github/eugene-babichenko/fixit)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/eugene-babichenko/fixit)

`fixit` is a terminal application that fixes mistakes in your commands inspired
by [The Fuck][thefuck]. It is also designed to be _fast as fuck_ (more about
that in the [motivation section](MOTIVATION.md)).

See contributing guidelines [here](CONTRIBUTING.md). If you want to help `fixit`
move forward, see the [roadmap](ROADMAP.md).

![demo](doc/demo.svg)

## How it works?

When you run the `fix` command, it gets the last command from the shell history.
Then it gets the output of this command in one of the two ways:

- Re-running the command.
- Retrieving the output from the emulator/multiplexer API (see
  [the list](#software-with-quick-fixes-available) of compatible software).

Once `fixit` has the command output, it runs the command and its output through
a number of [rules](#available-rules) to determine appropriate fixes. After you
select a fix it is run automatically and added to your shell history.

## Installation

<details>
<summary>apt (Ubuntu, Debian, Mint, etc)</summary>

    echo "deb [arch=$(dpkg --print-architecture) trusted=yes] https://eugene-babichenko.github.io/fixit/ppa ./" | sudo tee /etc/apt/sources.list.d/fixit.list > /dev/null
    sudo apt update
    sudo apt install fixit

</details>

<details>
<summary>Arch Linux</summary>

    yay -S fixit-bin

Or build from source:

    yay -S fixit

</details>

<details>
<summary>Fedora, RHEL, etc (everything using dnf, yum, rpm)</summary>

Create a new file with the following contents at `/etc/yum.repos.d/fixit.repo`

    [fixit]
    name=fixit GitHub repository
    baseurl=https://eugene-babichenko.github.io/fixit/rpm
    enabled=1
    gpgcheck=0

Run `dnf install fixit`.

</details>

<details>
<summary>macOS Homebrew/Linuxbrew</summary>

    brew install eugene-babichenko/fixit/fixit

</details>
<details>
<summary>Cargo (any OS, you will need the Rust toolchain)</summary>

    cargo install fixit-cli

</details>

You can also download pre-built binaries for Linux (static binaries) and macOS
from [Releases][releases].

## Shell setup

Add the corresponding line to your shell configuration file.

**bash**:

    eval "$(fixit init bash)"

**zsh**:

    eval "$(fixit init zsh)"

**fish**:

    fixit init fish | source

**Powershell**:

    Invoke-Expression (fixit init powershell | Out-String)

## Usage

Having a command that broke? Just type `fix` in your shell.

## A note for `kitty` users

You don't need to do this, if you use a terminal multiplexer inside `kitty`.

This is optional, but without this `fixit` will fall back to just re-running the
command, which is going to be slower.

To make quick completions work, you need to enable [remote
control][kitty-remote]. This is recommended, because this application uses
`kitty @ get-text` to retrieve the command output. For the best performance and
stability you are advised to set up [shell integration][kitty-sh-i].

## Software with quick fixes available

Terminal multiplexers:

- tmux
- Zellij

Teminal emulators:

- iTerm
- kitty
- Wezterm
- Terminal.app

## Configuration

### Alias name

You can change the name of the generated alias by providing the `--name`
argument to the `init` command:

    fixit init --name f fish | source

This will generate the alias named `f` instead of `fix`.

### Fixing

Environment variables:

| Variable                   | Description                                                                                         | Default Value |
| -------------------------- | --------------------------------------------------------------------------------------------------- | ------------- |
| `FIXIT_PAGE_SIZE`          | Controls how many suggestions per page you will see on the screen                                   | 5             |
| `FIXIT_QUICK_ENABLE`       | Enable quick fixes using terminal emulator/multiplexer API                                          | `true`        |
| `FIXIT_QUICK_SEARCH_DEPTH` | Sets the number of lines to get from the scrollback buffer in addition to what we see on the screen | 1000          |

This configuration is applied immediately, meaning you do not need to
re-initialize.

### Logging

Logging is implemented via `env_logger`. Please refer to its
[documentation][env-logger] to see how to configure the logs.

## Available rules

- `brew_update_upgrade` - replace `brew update` with `brew upgrade` when trying
  to update a Homebrew package.
- `cargo_clippy_args`: a rule for `cargo clippy` arguments that must be
  separated by `--`.
- `cargo_install_cwd` - fix `cargo install` without arguments (it requires
  `--path`).
- `cargo_wrong_command` - fix misspelled cargo commands.
- `command_not_found` - search for misspelled command through `$PATH`.
- `cp_cwd` - `cp` came with only one argument, maybe you want to copy to the
  current dir?
- `cp_dir` - add `-R` to `cp` when you are attempting to copy a directory.
- `git_add_all_lowercase` - correct `git add -a` to `git add -A`.
- `git_commit_no_changes` - suggest using `git commit -a`.
- `git_no_upstream` - set upstream branch when pushing.
- `git_retag` - suggest deleting a git tag and tagging a new commit with it, if
  the tag already exists.
- `git_wrong_command` - fix misspelled git commands.
- `mkdir_missing_parent` - suggest using `mkdir -p` to create missing in-between
  directories.
- `rm_dir` - add `-r` to `rm` when trying to remove a directory.
- `sudo` - prepend with `sudo` when permission to execute a command was denied.
- `taskfile_no_task` - suggest task names when trying to run a task from
  `Taskfile`.
- `uv_unexpected_argument` - fix typos in arguments for the `uv` Python package
  manager.

[thefuck]: https://github.com/nvbn/thefuck
[env-logger]:
  https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
[kitty-remote]:
  https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.allow_remote_control
[kitty-sh-i]: https://sw.kovidgoyal.net/kitty/shell-integration/
[releases]: https://github.com/eugene-babichenko/fixit/releases
