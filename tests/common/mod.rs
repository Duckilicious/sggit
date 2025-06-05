use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use anyhow::Result;

pub struct TestEnv {
    pub work_dir: TempDir,
    pub remote_dir: TempDir,
}

impl TestEnv {
    pub fn new() -> Result<Self> {
        let work_dir = TempDir::new()?;
        let remote_dir = TempDir::new()?;
        
        Ok(TestEnv {
            work_dir,
            remote_dir,
        })
    }

    pub fn work_path(&self) -> &Path {
        self.work_dir.path()
    }

    pub fn remote_path(&self) -> &Path {
        self.remote_dir.path()
    }

    pub fn create_remote_file<P: AsRef<Path>>(&self, relative_path: P, content: &str) -> Result<PathBuf> {
        let file_path = self.remote_dir.path().join(relative_path);
        
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&file_path, content)?;
        Ok(file_path)
    }

    pub fn create_work_file<P: AsRef<Path>>(&self, relative_path: P, content: &str) -> Result<PathBuf> {
        let file_path = self.work_dir.path().join(relative_path);
        
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&file_path, content)?;
        Ok(file_path)
    }

    pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }
}