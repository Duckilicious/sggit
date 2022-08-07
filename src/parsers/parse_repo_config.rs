use crate::common_helpers;
use crate::parsers::parse_platform_setting::PlatformConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path;

#[derive(Deserialize, Serialize)]
pub struct RepoConfig {
    files: Vec<FileDescriptor>,
}

#[derive(Deserialize, Serialize, Clone)]
struct FileDescriptor {
    path_in_repo: path::PathBuf,
    platforms: Vec<Platform>,
}

#[derive(Deserialize, Serialize, Clone)]
struct Platform {
    name: String,
    path: path::PathBuf,
}

impl FileDescriptor {
    fn new(path_in_repo: path::PathBuf, platforms: Vec<Platform>) -> Self {
        FileDescriptor {
            path_in_repo,
            platforms,
        }
    }
}

impl Platform {
    fn new(name: String, path: path::PathBuf) -> Self {
        Platform { name, path }
    }
}

impl RepoConfig {
    pub fn parse_repo_config(path_to_repo: &path::Path) -> Self {
        let path_to_config = path_to_repo.join(common_helpers::REPO_CONFIG_FILE);
        let res = fs::read_to_string(path_to_config)
            .unwrap_or_else(|err| panic!("Failed to read repo config from path {}", err));
        let cfg: RepoConfig =
            serde_json::from_str(res.as_str()).expect("Failed to seriealize repo config");

        cfg
    }

    pub fn get_src_dst_all_files(&self, curr_platform: &str) -> Vec<(&path::Path, &path::Path)> {
        let mut srcs_dsts = Vec::new();

        for file_desc in &self.files {
            let dst = file_desc.path_in_repo.as_path();
            let src = file_desc
                .platforms
                .iter()
                .find(|&platform| platform.name == curr_platform);
            if src.is_none() {
                continue;
            }

            let src = src.unwrap().path.as_path();
            srcs_dsts.push((src, dst));
        }

        srcs_dsts
    }

    pub fn create_initial_repo_config(platform_config: &PlatformConfig) {
        let platform = platform_config.get_platform();
        let repo_path = platform_config.get_repo_path();

        let platform_directory_name = platform.to_string() + "_only";
        let platform_directony_path = repo_path.join(path::Path::new(&platform_directory_name));
        fs::create_dir(&platform_directony_path).unwrap_or_else(|err| {
            panic!(
                "Failed to create directory {}, {}",
                platform_directony_path.to_string_lossy(),
                err
            )
        });

        let platform_config_platform_desc = Platform {
            name: platform.to_string(),
            path: path::PathBuf::from(common_helpers::SGGIT_PATH.to_string()),
        };
        let repo_config_platform_desc = Platform {
            name: platform.to_string(),
            path: repo_path.join(common_helpers::REPO_CONFIG_FILE),
        };

        let platform_config_file_desc = FileDescriptor {
            path_in_repo: path::PathBuf::from(platform_directory_name + "/sggit.json"),
            platforms: vec![platform_config_platform_desc],
        };
        let repo_config_file_desc = FileDescriptor {
            path_in_repo: path::PathBuf::from(common_helpers::REPO_CONFIG_FILE),
            platforms: vec![repo_config_platform_desc],
        };

        let repo_config = RepoConfig {
            files: vec![platform_config_file_desc, repo_config_file_desc],
        };
        repo_config.save(repo_path);
    }

    fn save(&self, repo_path: &path::Path) {
        std::fs::write(
            repo_path.join(common_helpers::REPO_CONFIG_FILE),
            serde_json::to_string_pretty(self).expect("Failed to seriealize repo config"),
        )
        .expect("Failed to save repo_config.json");
    }

    fn append_file(&mut self, file_desc: FileDescriptor) {
        self.files.push(file_desc);
    }

    fn remove_file_desc_by_path(&mut self, repo_path: &path::Path) {
        let entry = self.files.iter().position(|file| *file.path_in_repo == *repo_path);
        self.files.remove(entry.expect("File to untrack isn't found"));
    }

    pub fn append_new_file_desc_to_repo_config(
        platform_config: &PlatformConfig,
        path_to_file: path::PathBuf,
        path_in_repo: path::PathBuf,
    ) {
        let repo_path = platform_config.get_repo_path();
        let file_desc = FileDescriptor::new(
            path_in_repo,
            vec![Platform::new(
                platform_config.get_platform().to_string(),
                path_to_file,
            )],
        );

        let mut repo_config = RepoConfig::parse_repo_config(platform_config.get_repo_path());
        repo_config.append_file(file_desc);
        repo_config.save(repo_path);
    }

    pub fn remove_file_from_repo_config(
        platform_config: &PlatformConfig,
        path_in_repo: &path::Path,
    ) {
        let repo_path = platform_config.get_repo_path();
        let mut repo_config = RepoConfig::parse_repo_config(platform_config.get_repo_path());
        repo_config.remove_file_desc_by_path(path_in_repo);
        repo_config.save(repo_path);
    }
}
