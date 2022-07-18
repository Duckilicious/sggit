use crate::command::{Command, CommandError};
use crate::parse_platform_setting::PlatformConfig;
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
    
}

impl Command for Init {
    //TODO: Fix error
    fn run_command(platform_config: Option<&PlatformConfig>) -> Result<(), CommandError> {
        Init::init_repo(platform_config.unwrap().get_repo_path())?;

        Ok(())
    }
}
