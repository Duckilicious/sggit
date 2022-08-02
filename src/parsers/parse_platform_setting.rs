use serde::{Deserialize, Serialize};
use std::fs;
use std::path;

#[derive(Deserialize, Serialize, Debug)]
pub struct PlatformConfig {
    platform: String,
    repo_path: path::PathBuf,
}

impl PlatformConfig {
    pub fn parse_platform_config() -> Option<Self> {
        let res = fs::read_to_string(crate::common_helpers::SGIT_PATH.to_string());
        match res {
            Err(_) => None,
            Ok(res) => {
                let platform_setting: Option<Self> = serde_json::from_str(res.as_str()).ok();
                platform_setting
            }
        }
    }

    pub fn new(platform: String, repo_path: path::PathBuf) -> Self {
        PlatformConfig {
            platform,
            repo_path,
        }
    }

    pub fn get_platform(&self) -> &str {
        self.platform.as_str()
    }

    pub fn get_repo_path(&self) -> &path::Path {
        &self.repo_path
    }
}
