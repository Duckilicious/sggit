use crate::commands::command::{Command, NoArgs};
use crate::commands::commit::{Commit, CommitArgs};
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config;
use std::path;
use std::process;

extern crate tilde_expand;

pub struct Init;

impl Init {
    fn init_repo(path: &path::Path) {
        std::fs::create_dir_all(path).unwrap_or_else(|err| {
            panic!(
                "Failed to create repo directory {} {}",
                path.to_str().unwrap(),
                err
            )
        });

        process::Command::new("git")
            .args(["init"])
            .current_dir(path)
            .output()
            .expect("Failed to init a new repo");
    }

    fn create_platform_setting() -> PlatformConfig {
        use std::io::stdin;
        println!("Enter platform name:");
        let mut platform = String::new();
        stdin()
            .read_line(&mut platform)
            .expect("Failed to read platform name");

        println!("Enter where you'd like to create your repo:");
        let mut repo_path = String::new();
        stdin()
            .read_line(&mut repo_path)
            .expect("Failed to read platform name");
        let repo_path = path::PathBuf::from(
            String::from_utf8(tilde_expand::tilde_expand(repo_path.trim().as_bytes()))
                .expect("Unable to parse pah into a string"),
        );

        let platform_setting = PlatformConfig::new(platform.trim().to_string(), repo_path);
        let platform_setting_serialized = serde_json::to_string_pretty(&platform_setting)
            .unwrap_or_else(|err| panic!("Failed to seriealize platform_setting {}", err));

        std::fs::write(
            crate::common_helpers::SGGIT_PATH.to_string(),
            platform_setting_serialized,
        )
        .unwrap_or_else(|err| panic!("Failed to create platform setting {}", err));

        platform_setting
    }

    fn create_repo_config(platform_config: &PlatformConfig) {
        parse_repo_config::RepoConfig::create_initial_repo_config(platform_config);
    }

    fn initial_commit(platform_config: &PlatformConfig) {
        Commit::run_command(Some(platform_config), Some(CommitArgs::new("Initial commit")));
    }
}

impl Command<NoArgs> for Init {
    fn run_command(platform_config: Option<&PlatformConfig>, _: Option<NoArgs>) {
        if let Some(_config) = platform_config {
            panic!("An exisiting platform config already exists")
        }

        let platform_config = Init::create_platform_setting();
        Init::init_repo(platform_config.get_repo_path());
        Init::create_repo_config(&platform_config);
        Init::initial_commit(&platform_config);
    }
}
