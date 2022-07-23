use crate::commands::{Command, CommandError};
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config;
use crate::commands::commit;
use git2::Repository;
use std::path;

pub struct Init;

impl Init {
    fn init_repo(path: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let err_on_open_existing_repo = Repository::open(path).err();
        if let None = err_on_open_existing_repo {
            return Err(Box::new(CommandError::from("Repo already exists in that location".to_string())));
        }

        Repository::init(path)?;
        Ok(())
    }

    fn create_platform_setting() -> Result<PlatformConfig, Box<dyn std::error::Error>> {
        use std::io::stdin;
        println!("Enter platform name:");
        let mut platform = String::new();
        stdin().read_line(&mut platform)
            .ok()
            .expect("Failed to read platform name");

        println!("Enter where you'd like to create your repo:");
        let mut repo_path = String::new();
        stdin().read_line(&mut repo_path)
            .ok()
            .expect("Failed to read platform name");
        let repo_path = path::PathBuf::from(repo_path.trim());

        let platform_setting = PlatformConfig::new(platform.trim().to_string(), repo_path);
        let platform_setting_serialized = serde_json::to_string_pretty(&platform_setting)?;

        std::fs::write(crate::common_helpers::SGIT_PATH.to_string(), platform_setting_serialized)?;
        Ok(platform_setting)
    }

    fn create_repo_config(platform_config: &PlatformConfig) -> Result<(), Box<dyn std::error::Error>> {
        parse_repo_config::RepoConfig::create_initial_repo_config(platform_config)?;
        Ok(())
    }

    fn initial_commit(platform_config: &PlatformConfig) -> Result<(), Box<dyn std::error::Error>> {
        commit::Commit::run_command(Some(platform_config))?;
        Ok(())
    }
}

impl Command for Init {
    fn run_command(platform_config: Option<&PlatformConfig>) -> Result<(), CommandError> {
        if let Some(_config) = platform_config {
            return Err(CommandError::from("An exisiting platform config already exists".to_string()));
        }

        let platform_config = Init::create_platform_setting()?;
        Init::init_repo(platform_config.get_repo_path())?;
        Init::create_repo_config(&platform_config)?;
        Init::initial_commit(&platform_config)?;
        Ok(())
    }
}
