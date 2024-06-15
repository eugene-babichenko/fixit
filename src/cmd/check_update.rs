use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
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
    result: Option<String>,
    #[serde(skip)]
    init: bool,
}

const RELEASE_ROUTE: &str = "https://api.github.com/repos/eugene-babichenko/fixit/releases/latest";

impl Cmd {
    fn run(self) -> Option<String> {
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

        let remote_version = match Version::parse(
            release
                .tag_name
                .strip_prefix('v')
                .unwrap_or(&release.tag_name),
        ) {
            Ok(remote_version) => remote_version,
            Err(err) => {
                log::debug!("the tag is not a valid version number: {err}");
                return None;
            }
        };

        // Cargo validates package version to be in semver format
        let local_version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
        if local_version < remote_version {
            Some(remote_version.to_string())
        } else {
            None
        }
    }

    pub fn check(self) {
        let Some(p) = update_file_path() else {
            log::debug!("could not get update file path");
            return;
        };

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let res = File::open(p.clone())
            .ok()
            .and_then(|file| serde_json::from_reader(file).ok())
            .unwrap_or(CheckResult {
                result: None,
                timestamp: time,
                init: true,
            });

        if let Some(update) = res.result {
            eprintln!("fixit: new version is available: {update}");
        }

        if !res.init && time.checked_sub(res.timestamp) <= Some(self.interval) {
            log::debug!("too early");
            return;
        }

        let res = self.run();

        let dir = p.parent().unwrap();
        let _ = create_dir_all(dir);
        let file = match File::create(p) {
            Ok(f) => f,
            Err(err) => {
                log::debug!("failed to open update.json: {err}");
                return;
            }
        };
        let _ = serde_json::to_writer(
            file,
            &CheckResult {
                result: res,
                timestamp: time,
                init: false,
            },
        );
    }
}

fn update_file_path() -> Option<PathBuf> {
    let mut p = dirs::data_local_dir()?.to_path_buf();
    p.push("fixit");
    p.push("update.json");
    Some(p)
}
