use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub remote_path: PathBuf,
    pub platform: String,
    pub local_path: PathBuf,
    pub last_synced: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SggitConfig {
    pub files: HashMap<String, Vec<FileEntry>>,
}

impl SggitConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = PathBuf::from(".sggit/config.json");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(".sggit")?;
        let config_path = PathBuf::from(".sggit/config.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn add_file(&mut self, remote_path: PathBuf) -> anyhow::Result<()> {
        let platform = std::env::consts::OS.to_string();
        let file_name = remote_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
            .to_string();
        
        let local_path = PathBuf::from(&file_name);
        
        let entry = FileEntry {
            remote_path,
            platform,
            local_path,
            last_synced: None,
        };

        self.files.entry(file_name).or_insert_with(Vec::new).push(entry);
        Ok(())
    }
}