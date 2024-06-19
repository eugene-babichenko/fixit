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

#[derive(Serialize, Deserialize)]
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
    pub fn check(self) {
        let Some(p) = update_file_path() else {
            log::debug!("could not get update file path");
            return;
        };

        let res = check(
            p.as_path(),
            env!("CARGO_PKG_VERSION"),
            RELEASE_ROUTE,
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
    release_route: &str,
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

    let Some(git_tag) = fetch_git_tag(release_route) else {
        log::error!("failed to fetch the latest git tag");
        write_update_file(update_file, &res);
        return res.result.map(|v| v.to_string());
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

    let res = CheckResult {
        result: res,
        timestamp: time,
        init: false,
    };

    write_update_file(update_file, &res);

    res.result.map(|v| v.to_string())
}

fn write_update_file(update_file: &Path, content: &CheckResult) {
    let dir = update_file.parent().unwrap();
    let _ = create_dir_all(dir);
    let file = match File::create(update_file) {
        Ok(f) => f,
        Err(err) => {
            log::error!("failed to open {}: {err}", update_file.display());
            return;
        }
    };
    if let Err(err) = serde_json::to_writer(file, content) {
        log::error!("failed to write {}: {err}", update_file.display());
    }
}

fn fetch_git_tag(route: &str) -> Option<String> {
    let response = match ureq::get(route).call() {
        Ok(response) => response,
        Err(err) => {
            log::error!("couldn't get the latest release: {err}");
            return None;
        }
    };

    let release: Release = match response.into_json() {
        Ok(release) => release,
        Err(err) => {
            log::error!("couldn't decode the response: {err}");
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
    use std::{
        io::{Seek, Write},
        str::FromStr,
    };

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
    #[case("", Some("0.3.0-beta"), true, true)]
    #[case("{\"result\": null, \"timestamp\": 0}", Some("0.3.0-beta"), true, true)]
    #[case(
        "{\"result\": \"0.2.1-beta\", \"timestamp\": 0}",
        Some("0.3.0-beta"),
        true,
        true
    )]
    #[case("", Some("0.2.0-alpha"), false, true)]
    #[case(
        "{\"result\": null, \"timestamp\": 0}",
        Some("0.2.0-alpha"),
        false,
        true
    )]
    #[case(
        "{\"result\": \"0.2.0-alpha\", \"timestamp\": 0}",
        Some("0.2.0-alpha"),
        false,
        true
    )]
    #[case(&format!("{{\"result\": \"0.2.0-alpha\", \"timestamp\": {} }}", curr_time()), Some("0.2.0-alpha"), false, false)]
    #[case(&format!("{{\"result\": \"0.3.0-alpha\", \"timestamp\": {} }}", curr_time()), Some("0.3.0-alpha"), true, false)]
    #[case("", None, false, true)]
    #[case("", Some("x0.3.0-alpha"), false, true)]
    fn update(
        #[case] update_file_contents: &str,
        #[case] git_tag: Option<&str>,
        #[case] update_expected: bool,
        #[case] request_expected: bool,
    ) {
        let server = httpmock::MockServer::start();
        let update_mock = server.mock(|when, then| {
            let r = git_tag.map(|t| Release {
                tag_name: t.to_string(),
            });
            when.method("GET").path("/release");
            then.status(200).json_body_obj(&r);
        });

        let f = NamedTempFile::new().unwrap();
        f.as_file()
            .write_all(update_file_contents.as_bytes())
            .unwrap();

        let r = check(f.path(), LOCAL_VERSION, &server.url("/release"), INTERVAL);

        if request_expected {
            update_mock.assert();
        }

        if update_expected {
            f.as_file().rewind().unwrap();
            let c: CheckResult = serde_json::from_reader(f.as_file()).unwrap();
            assert_eq!(git_tag, r.as_deref());
            assert_eq!(c.result, git_tag.map(|t| Version::from_str(&t).unwrap()));
            assert_eq!(c.init, false);
        } else {
            assert_eq!(None, r);
        }
    }

    #[test]
    fn test_update_request_with_real_data() {
        let server = httpmock::MockServer::start();
        let update_mock = server.mock(|when, then| {
            when.method("GET").path("/release");
            then.status(200).body_from_file("tests/data/release.json");
        });
        let tag = fetch_git_tag(&server.url("/release"));
        update_mock.assert();
        assert_eq!(Some("v0.3.1-beta"), tag.as_deref());
    }

    #[test]
    fn test_update_request_no_response() {
        let server = httpmock::MockServer::start();
        let tag = fetch_git_tag(&server.url("/release"));
        assert_eq!(None, tag);
    }
}
