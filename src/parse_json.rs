use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io;
use std::path;

#[derive(Debug)]
pub struct ParseError {
    err: ParseErrorImpl,
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        ParseError {
            err: ParseErrorImpl::ParseJsonError(err),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError {
            err: ParseErrorImpl::IoError(err),
        }
    }
}

#[derive(Debug)]
enum ParseErrorImpl {
    ParseJsonError(serde_json::Error),
    IoError(io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.err {
            ParseErrorImpl::ParseJsonError(err) => {
                println!("Failed to parse json - Bad formt, {}", err)
            }
            ParseErrorImpl::IoError(err) => println!("Failed to open config json, {}", err),
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct RepoConfig {
    files: Vec<FileDescriptor>,
}

#[derive(Deserialize, Serialize)]
struct FileDescriptor {
    path_in_repo: path::PathBuf,
    platforms: Vec<Platform>,
}

#[derive(Deserialize, Serialize)]
struct Platform {
    name: String,
    path: path::PathBuf,
}

impl RepoConfig {
    pub fn parse_repo_config() -> Result<Self, ParseError> {
        let res = fs::read_to_string("./example.json")?;
        let cfg : RepoConfig = serde_json::from_str(res.as_str())?; 

        Ok(cfg)
    }

    pub fn get_src_dst_all_files(&self, curr_platform : String) -> Vec<(&path::Path, &path::Path)> {
        let mut srcs_dsts = Vec::new();

        for file_desc in &self.files {
            let dst = file_desc.path_in_repo.as_path();
            let src = file_desc.platforms.iter().find(|&platform| platform.name == curr_platform);
            if let None = src {
                continue;
            }

            let src = src.unwrap().path.as_path();
            srcs_dsts.push((src, dst));
        }

        srcs_dsts
    }
}
