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
        let res = fs::read_to_string("./platform_setting.json")?;
        let cfg: PlatformConfig = serde_json::from_str(res.as_str())?;
        Ok(cfg)
    }
    
    pub fn get_platform(&self) -> &str {
        self.platform.as_str()
    }

    pub fn get_repo_path(&self) -> &path::Path {
        &self.repo_path
    }
}
