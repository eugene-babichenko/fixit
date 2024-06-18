# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- New rule: `git_add_all_lowercase` - correct `git add -a` to `git add -A`.

### Changed

- Respect `FIXIT_UPDATE_CHECK_INTERVAL` when an updated fails.

### Fixed

- Search results deduplication.

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
