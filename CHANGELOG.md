# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `cargo_clippy_args`: a rule for `cargo clippy` arguments that must be
  separated by `--`.
- Parallel search for fixes via `rayon`.
- rules: added `uv_unexpected_argument` for fixing errors in the Python `uv`
  package manager.

### Changed

- Logging is now controlled with `RUST_LOG` and `RUST_LOG_STYLE`.

## v0.9.0

### Added

- Support for quick fixes in Apple Terminal.

### Changed

- Simplify internal Powerhell workflow.

### Fixed

- Quick search not working properly with bash.
- Getting text in iTerm with shell integration enabled.
- Improper processing of quick search when bash outputs errors in the format of
  `bash: line 1: error text goes here`.
- `kitty` would always fall back to re-running the last command.

## v0.8.1

### Fixed

- Reliably detect running inside Powershell.

## v0.8.0

### Added

- PowerShell support.

### Fixed

- Panic when fix is called with empty history.

## v0.7.0

### Added

- rules: `command_not_found` correctly processes ellipsis found in some Bash
  implementations.

### Changed

- Remove dependency on `itertools`.

### Removed

- Progress bar: this software is usually so fast the progress bar is not needed
  anyways.

## v0.6.0

### Added

- Support for quick fixes in Zellij.

### Fixed

- Getting shell functions and aliases in fish.

## v0.5.1

### Added

- rules: `command_not_found` now includes shell functions and aliases into the
  search.

## v0.5.0

### Added

- Support for getting command output from iTerm2.
- rules: `sudo` now reacts to `operation not permitted` messages.
- rules: react to `nothing added to commit but untracked files present` with
  `git commit`.

### Fixed

- rules: `cp_cwd` false positives.

### Removed

- Update checker: it is hard to test, bloats the dependencies and is completely
  unnecessary given package managers.

## v0.4.0-beta

### Added

- New rule: `git_add_all_lowercase` - correct `git add -a` to `git add -A`.

### Changed

- Respect `FIXIT_UPDATE_CHECK_INTERVAL` when an updated fails.

### Fixed

- Search results deduplication.
- `command_not_found`: bash command detection.

## v0.3.1-beta

### Fixed

- Update checking.

## v0.3.0-beta

### Added

- `cp_dir` - fix attempting to `cp` a directory without `-R`.
- `cp_cmd` - if `cp` was given only one argument assume we wanted to copy in the
  current dir.

### Fixed

- Fix update logic.
- Make most rules independent of argument placement.
- On fish searcher could stumble upon the line with fish native error highlight.
- Correctly operate on complex commands.

## v0.2.0-alpha

### Added

- Automatic update notifications

## v0.1.0-alpha

### Added

- Initial release
