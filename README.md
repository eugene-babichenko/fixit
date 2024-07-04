# `fixit`

![GitHub Release](https://img.shields.io/github/v/release/eugene-babichenko/fixit)
[![Build status](https://github.com/eugene-babichenko/fixit/actions/workflows/tests.yml/badge.svg)](https://github.com/eugene-babichenko/fixit/actions)
[![Coverage Status](https://coveralls.io/repos/github/eugene-babichenko/fixit/badge.svg)](https://coveralls.io/github/eugene-babichenko/fixit)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/eugene-babichenko/fixit)

`fixit` is a terminal application that fixes mistakes in your commands inspired
by [The Fuck][thefuck]. It is also designed to be _fast as fuck_ (more about
that in the ["Why?" section](#why)).

See contributing guidelines [here](CONTRIBUTING.md). If you want to help `fixit`
move forward, see the [roadmap](#roadmap).

![demo](doc/demo.svg)

## How it works?

When you run the `fix` command, it gets the last command from the shell history.
Then one of two things happen:

- The previous command is re-run to get its output.
- The output of the previous command is retrieved via your terminal
  emulator/multiplexer API (available on tmux, kitty and WezTerm).

Once `fixit` has the command output, it runs the command and its output through
a number of [rules](#available-rules) to determine appropriate fixes. After you
select a fix it is run automatically and added to your shell history.

## Installation

Any Linux distro that uses `apt` (Ubuntu, Debian, Mint, etc):

    echo "deb [arch=$(dpkg --print-architecture) trusted=yes] https://eugene-babichenko.github.io/fixit/ppa ./" | sudo tee /etc/apt/sources.list.d/fixit.list > /dev/null
    sudo apt update
    sudo apt install fixit

For Arch Linux users fixit is [available][aur] on AUR (build from source):

    yay -S fixit

`x86_64` and `aarch64` pre-built binaries are available as well:

    yay -S fixit-bin

Using Homebrew/Linuxbrew:

    brew install eugene-babichenko/fixit/fixit

Installing with Cargo (you will need the [Rust toolchain][rust]):

    cargo install fixit-cli

You can also download pre-built binaries for Linux (static binaries) and macOS
from [Releases][releases].

## Shell setup

Add the corresponding line to your shell configuration file

**bash**:

    eval "$(fixit init bash)"

**zsh**:

    eval "$(fixit init zsh)"

**fish**:

    fixit init fish | source

## Teminal emulator/multiplexer setup

### kitty

You can skip this section if you are using tmux inside Kitty. Quick completions
work out of the box with tmux.

This is optional, but without this `fixit` will fall back to just re-running the
command, which is going to be slower.

To make quick completions work, you need to enable [remote
control][kitty-remote]. This is required, because this application uses
`kitty @ get-text` to retrieve the command output. For the best performance and
stability you are advised to set up [shell integration][kitty-sh-i].

### Other software

You do not need any additional setup.

## Usage

Having a command that broke? Just type `fix`.

## Configuration

### Initialization

You can change the name of the alias by providing the `--name` argument to the
`init` command:

    fixit init --name f fish | source

This will generate the command named `f` instead of `fix`.

### Fixing

Environment variables:

- `FIXIT_PAGE_SIZE` regulates how many suggestions per page you will see. The
  default is `5`.
- `FIXIT_QUICK_ENABLE` - when running inside a supported terminal
  emulator/multiplexer, try to get the command output with its API instead of
  re-running the given command. This is generally much faster, so it is
  recommended that you leave it as is unless you run into any bugs associated
  with finding fixes. The combination that can be potentially buggy is
  suppported terminal emulator with unsupported multiplexer when the failed
  command is not visible on the screen. The default value is `true`. Pass
  `false` to disable.
- `FIXIT_QUICK_SEARCH_DEPTH` sets the number of lines to get from the scrollback
  buffer in addition to what we see on the screen. The default is `1000`.

### Update checks

`fixit` can check for updates and notify you about them. By default it does so
once a day. The check is done by querying GitHub releases API. `fixit` does not
collect any data about you.

- `FIXIT_UPDATE_CHECK_INTERVAL` - the update check interval in secons.
- `FIXIT_UPDATE_CHECK_ENABLE` - enable update checks. Set to `false` to disable.

### Logging

Environment variables:

- `FIXIT_LOG` controls logging. The default log level is `error`. For
  development purposes you generally need to enable the `debug` level
  (`FIXIT_LOG=debug`). The logger is pretty flexible and you can learn more from
  the [`env_logger` documentation page][env-logger].
- `FIXIT_LOG_STYLE` controls logger styling. Refer to `env_logger` docs for this
  as well.

## Available rules

- `brew_update_upgrade` - replace `brew update` with `brew upgrade` when trying
  to update a Homebrew package.
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
- `git_wrong_command` - fix misspelled git commands.
- `mkdir_missing_parent` - suggest using `mkdir -p` to create missing in-between
  directories.
- `rm_dir` - add `-r` to `rm` when trying to remove a directory.
- `sudo` - prepend with `sudo` when permission to execute a command was denied.

## Roadmap

- [x] Automatic update check
- [x] Deal with complex commands (e.g. env var specification included:
      `FOO=bar command -arg`)
- Quick suggestions without re-running commands:
  - Via terminal emulator/multiplexer API
    - [x] tmux
    - [x] kitty
      - [x] Get only the last command output with shell integration.
    - [x] WezTerm
    - [x] iTerm2
    - [ ] Zellij
  - [ ] Get terminal logs by running an arbitrary command.
  - [ ] Get the output of the previous command by running an arbitrary command.
  - [ ] Wrap around shell to read its logs (a la The Fuck instant mode).
  - [ ] Recognize OSC 133 escape sequences. This will help to determine command
        output boundaries.
- Shell support
  - [x] bash
  - [x] zsh
  - [x] fish
  - [ ] Powershell
- Automated testing
  - [ ] Integration tests supported shells (headless)
  - [ ] Integration tests for supported emulators/multiplexers.
- [ ] Optional removal of a failed command from history.
- Packaging
  - [x] Homebrew
  - [x] AUR
  - [x] deb
  - [ ] rpm
  - [ ] NixOS
  - anything else

## Why?

While The Fuck is certainly magnificient, it does have a fatal flaw: it is
written in Python. With all due respect, Python is slow and this does harm the
user experience in two ways:

- It creates a perceivable and annoying slowdown during the shell startup,
  because it is written in Python.
- The fixes themselves can be rather slow.

On top of that, sometimes system-wide Python packages just break. In fact, this
happened to me while I've been writing this page and trying to do benchmarks.

The intention behind `fixit` is to solve this by the re-write in a natively
compiled language. Namely, in Rust. This removes the overhead of the Python
interpreter and opens up the potential to search for fixes utilizing all of the
CPU cores.

### On "instant mode"

The Fuck has a feature called "instant mode" where it wraps around your shell to
log output and read it instead of re-running the previous command. While this
approach is certainly useful and has the benefit of being available on every
terminal emulator locally, over SSH remotely, and without any additional
terminal configuration, I am not a big fan of it. Going this way can mess with
your shell output and creates a mess of nested processes. I am not totally
against it and would totally love if someone implements it for me, but for this
application the preferred way is to integrate with the terminal emulator API if
such option is available. The ones that I'm aware of with appropriate APIs are
WezTerm, kitty and iTerm2 .This way you do not create an additional layer
between a shell and a user and the fallback to just re-running a command is very
straightforward without editing shell configuration files.

[thefuck]: https://github.com/nvbn/thefuck
[rust]: https://www.rust-lang.org/tools/install
[env-logger]:
  https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
[kitty-remote]:
  https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.allow_remote_control
[kitty-sh-i]: https://sw.kovidgoyal.net/kitty/shell-integration/
[binstall]: https://github.com/cargo-bins/cargo-binstall
[aur]: https://aur.archlinux.org/packages/fixit
[releases]: https://github.com/eugene-babichenko/fixit/releases
