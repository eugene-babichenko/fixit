use std::{
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
pub struct Cmd {
    /// Update check interval in seconds.
    #[arg(env = "FIXIT_UPDATE_CHECK_INTERVAL", default_value_t = 86400)]
    interval: u64,
    /// Enable periodic update checking.
    #[arg(env = "FIXIT_UPDATE_CHECK_ENABLE", default_value_t = true)]
    enabled: bool,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[derive(Serialize, Deserialize)]
struct CheckResult {
    timestamp: u64,
    result: Option<Version>,
    #[serde(skip)]
    init: bool,
}

const RELEASE_ROUTE: &str = "https://api.github.com/repos/eugene-babichenko/fixit/releases/latest";

impl Cmd {
    #[cfg(not(tarpaulin_include))]
    pub fn check(self) {
        let Some(p) = update_file_path() else {
            log::debug!("could not get update file path");
            return;
        };

        let res = check(
            p.as_path(),
            env!("CARGO_PKG_VERSION"),
            fetch_git_tag,
            self.interval,
        );

        if let Some(res) = res {
            eprintln!("fixit: new version is available: {res}");
        }
    }
}

fn check(
    update_file: &Path,
    local_version: &str,
    git_tag: impl Fn() -> Option<String>,
    interval: u64,
) -> Option<String> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let res = File::open(update_file)
        .ok()
        .and_then(|file| serde_json::from_reader(file).ok())
        .unwrap_or(CheckResult {
            result: None,
            timestamp: time,
            init: true,
        });

    let local_version = Version::parse(local_version).unwrap();

    if !res.init && time.checked_sub(res.timestamp) <= Some(interval) {
        log::debug!("too early");
        if Some(local_version) < res.result {
            return res.result.map(|v| v.to_string());
        }
        return None;
    }

    let Some(git_tag) = git_tag() else {
        log::error!("failed to fetch the latest git tag");
        return None;
    };

    let remote_version = match Version::parse(git_tag.strip_prefix('v').unwrap_or(&git_tag)) {
        Ok(remote_version) => remote_version,
        Err(err) => {
            log::error!("the git tag is not a valid version number: {err}");
            return None;
        }
    };

    let res = if local_version < remote_version {
        Some(remote_version)
    } else {
        None
    };

    let dir = update_file.parent().unwrap();
    let _ = create_dir_all(dir);
    let file = match File::create(update_file) {
        Ok(f) => f,
        Err(err) => {
            log::error!("failed to open update.json: {err}");
            return None;
        }
    };

    let res = CheckResult {
        result: res,
        timestamp: time,
        init: false,
    };

    let _ = serde_json::to_writer(file, &res);

    res.result.map(|v| v.to_string())
}

#[cfg(not(tarpaulin_include))]
fn fetch_git_tag() -> Option<String> {
    let response = match ureq::get(RELEASE_ROUTE).call() {
        Ok(response) => response,
        Err(err) => {
            log::debug!("couldn't get the latest release: {err}");
            return None;
        }
    };

    let release: Release = match response.into_json() {
        Ok(release) => release,
        Err(err) => {
            log::debug!("couldn't decode the response: {err}");
            return None;
        }
    };

    Some(release.tag_name)
}

#[cfg(not(tarpaulin_include))]
fn update_file_path() -> Option<PathBuf> {
    let mut p = dirs::data_local_dir()?.to_path_buf();
    p.push("fixit");
    p.push("update.json");
    Some(p)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use rstest::rstest;
    use tempfile::NamedTempFile;

    use super::*;

    const INTERVAL: u64 = 1000000;
    const LOCAL_VERSION: &str = "0.2.0-alpha";

    fn curr_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[rstest]
    #[case("", "0.3.0-beta", true)]
    #[case("{\"result\": null, \"timestamp\": 0}", "0.3.0-beta", true)]
    #[case("{\"result\": \"0.2.1-beta\", \"timestamp\": 0}", "0.3.0-beta", true)]
    #[case("", "0.2.0-alpha", false)]
    #[case("{\"result\": null, \"timestamp\": 0}", "0.2.0-alpha", false)]
    #[case(
        "{\"result\": \"0.2.0-alpha\", \"timestamp\": 0}",
        "0.2.0-alpha",
        false
    )]
    #[case(&format!("{{\"result\": \"0.2.0-alpha\", \"timestamp\": {} }}", curr_time()), "0.2.0-alpha", false)]
    #[case(&format!("{{\"result\": \"0.3.0-alpha\", \"timestamp\": {} }}", curr_time()), "0.3.0-alpha", true)]
    fn update(#[case] update_file_contents: &str, #[case] git_tag: &str, #[case] expected: bool) {
        let git_tag_fn = || -> Option<String> {
            let mut git_tag = git_tag.to_string();
            git_tag.insert(0, 'v');
            Some(git_tag)
        };

        println!("{update_file_contents}");

        let f = NamedTempFile::new().unwrap();
        f.as_file()
            .write_all(update_file_contents.as_bytes())
            .unwrap();

        let r = check(f.path(), LOCAL_VERSION, git_tag_fn, INTERVAL);
        if expected {
            assert_eq!(Some(git_tag), r.as_deref());
        } else {
            assert_eq!(None, r);
        }
    }
}
