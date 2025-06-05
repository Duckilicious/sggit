use anyhow::Result;
use git2::{Repository, Signature};
use std::path::PathBuf;
use std::fs;
use chrono::{Utc, DateTime};

use crate::config::SggitConfig;

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
    let repo = Repository::open(".")?;
    
    let mut updated_files = Vec::new();
    let mut commit_message_lines = Vec::new();

    for (_file_name, entries) in &config.files {
        for entry in entries {
            if entry.platform == current_platform {
                if entry.remote_path.exists() {
                    let metadata = fs::metadata(&entry.remote_path)?;
                    let modified_time: DateTime<Utc> = metadata.modified()?.into();
                    
                    fs::copy(&entry.remote_path, &entry.local_path)?;
                    
                    let file_info = format!(
                        "- {} (modified: {})", 
                        entry.local_path.display(), 
                        modified_time.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    commit_message_lines.push(file_info);
                    updated_files.push(entry.local_path.clone());
                    
                    println!("Updated {} from {}", entry.local_path.display(), entry.remote_path.display());
                } else {
                    println!("Warning: {} does not exist", entry.remote_path.display());
                }
            }
        }
    }

    if !updated_files.is_empty() {
        let mut index = repo.index()?;
        
        for file_path in &updated_files {
            index.add_path(file_path)?;
        }
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        
        let sig = Signature::now("sggit", "sggit@localhost")?;
        
        let commit_message = format!(
            "Update {} file{} from remote locations\n\n{}",
            updated_files.len(),
            if updated_files.len() == 1 { "" } else { "s" },
            commit_message_lines.join("\n")
        );
        
        // Check if we have a HEAD commit (parent)
        let parents = if let Ok(head) = repo.head() {
            if let Some(target) = head.target() {
                vec![repo.find_commit(target)?]
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        
        let parent_refs: Vec<&git2::Commit> = parents.iter().collect();
        
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &commit_message,
            &tree,
            &parent_refs,
        )?;
        
        println!("Committed {} updated file{} to git repository", 
                updated_files.len(),
                if updated_files.len() == 1 { "" } else { "s" });
    } else {
        println!("No files updated");
    }

    Ok(())
}

pub fn sync_command() -> Result<()> {
    let mut config = SggitConfig::load()?;
    let current_platform = std::env::consts::OS;
    let _repo = Repository::open(".")?;

    for (_file_name, entries) in &mut config.files {
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