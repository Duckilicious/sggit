use crate::common_helpers;
use crate::parsers::parse_platform_setting::PlatformConfig;
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
    pub fn parse_repo_config(path_to_repo: &path::Path) -> Result<Self, ParseError> {
        let res = fs::read_to_string(path_to_repo.join(common_helpers::REPO_CONFIG_FILE))?;
        let cfg: RepoConfig = serde_json::from_str(res.as_str())?;

        Ok(cfg)
    }

    pub fn get_src_dst_all_files(&self, curr_platform: &str) -> Vec<(&path::Path, &path::Path)> {
        let mut srcs_dsts = Vec::new();

        for file_desc in &self.files {
            let dst = file_desc.path_in_repo.as_path();
            let src = file_desc
                .platforms
                .iter()
                .find(|&platform| platform.name == curr_platform);
            if let None = src {
                continue;
            }

            let src = src.unwrap().path.as_path();
            srcs_dsts.push((src, dst));
        }

        srcs_dsts
    }

    pub fn create_initial_repo_config(platform_config: &PlatformConfig) -> Result<(), Box<dyn std::error::Error>> {
        let platform = platform_config.get_platform();
        let repo_path = platform_config.get_repo_path();

        let platform_directory_name = platform.to_string() + "_only";
        let platform_directony_path = repo_path.join(path::Path::new(&platform_directory_name));
        fs::create_dir(platform_directony_path)?;

        let platform_config_platform_desc = Platform {
            name: platform.to_string(),
            path: path::PathBuf::from(common_helpers::SGIT_PATH.to_string()),
        };
        let repo_config_platform_desc = Platform {
            name: platform.to_string(),
            path: repo_path.join(common_helpers::REPO_CONFIG_FILE),
        };

        let platform_config_file_desc = FileDescriptor {
            path_in_repo: path::PathBuf::from(platform_directory_name + "/sgit.json"),
            platforms: vec![platform_config_platform_desc],
        };
        let repo_config_file_desc = FileDescriptor {
            path_in_repo: path::PathBuf::from(common_helpers::REPO_CONFIG_FILE),
            platforms: vec![repo_config_platform_desc],
        };

        let repo_config = RepoConfig {
            files: vec![platform_config_file_desc, repo_config_file_desc],
        };
        std::fs::write(
            repo_path.join(common_helpers::REPO_CONFIG_FILE),
            serde_json::to_string_pretty(&repo_config).expect("Failed to seriealize repo config"),
        )?;

        Ok(())
    }
}
