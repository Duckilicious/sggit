use serde::{Deserialize, Serialize};
use std::path;
use std::fs;

#[derive(Deserialize, Serialize,Debug)]
pub struct PlatformConfig {
    platform: String,
    repo_path: path::PathBuf,
}

impl PlatformConfig {
    pub fn parse_platform_config() -> Result<Self, Box<dyn std::error::Error>> {
        let res = fs::read_to_string(crate::common_helpers::SGIT_PATH.to_string())?;
        let cfg: PlatformConfig = serde_json::from_str(res.as_str())?;
        Ok(cfg)
    }

    pub fn new(platform: String, repo_path: path::PathBuf) -> Self {
        PlatformConfig { platform, repo_path }
    }
    
    pub fn get_platform(&self) -> &str {
        self.platform.as_str()
    }

    pub fn get_repo_path(&self) -> &path::Path {
        &self.repo_path
    }
}
