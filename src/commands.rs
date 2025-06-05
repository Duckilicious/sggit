use anyhow::Result;
use git2::Repository;
use std::path::PathBuf;
use std::fs;
use chrono::Utc;

use crate::config::{SggitConfig, FileEntry};

pub fn init_command() -> Result<()> {
    Repository::init(".")?;
    
    let config = SggitConfig::default();
    config.save()?;
    
    println!("Initialized empty sggit repository");
    Ok(())
}

pub fn add_command(remote_path: PathBuf) -> Result<()> {
    if !remote_path.exists() {
        return Err(anyhow::anyhow!("File does not exist: {:?}", remote_path));
    }

    let mut config = SggitConfig::load()?;
    config.add_file(remote_path.clone())?;
    config.save()?;

    println!("Added {} to sggit tracking", remote_path.display());
    Ok(())
}

pub fn update_command() -> Result<()> {
    let config = SggitConfig::load()?;
    let current_platform = std::env::consts::OS;

    for (file_name, entries) in &config.files {
        for entry in entries {
            if entry.platform == current_platform {
                if entry.remote_path.exists() {
                    fs::copy(&entry.remote_path, &entry.local_path)?;
                    println!("Updated {} from {}", entry.local_path.display(), entry.remote_path.display());
                } else {
                    println!("Warning: {} does not exist", entry.remote_path.display());
                }
            }
        }
    }

    Ok(())
}

pub fn sync_command() -> Result<()> {
    let mut config = SggitConfig::load()?;
    let current_platform = std::env::consts::OS;
    let repo = Repository::open(".")?;

    for (file_name, entries) in &mut config.files {
        for entry in entries.iter_mut() {
            if entry.platform == current_platform && entry.local_path.exists() {
                let should_sync = if entry.remote_path.exists() {
                    let remote_metadata = fs::metadata(&entry.remote_path)?;
                    let remote_modified = remote_metadata.modified()?;
                    
                    let local_metadata = fs::metadata(&entry.local_path)?;
                    let local_modified = local_metadata.modified()?;
                    
                    local_modified > remote_modified
                } else {
                    true
                };

                if should_sync {
                    if let Some(parent) = entry.remote_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(&entry.local_path, &entry.remote_path)?;
                    entry.last_synced = Some(Utc::now());
                    println!("Synced {} to {}", entry.local_path.display(), entry.remote_path.display());
                } else {
                    println!("Skipped {} (remote file is newer)", entry.remote_path.display());
                }
            }
        }
    }

    config.save()?;
    Ok(())
}