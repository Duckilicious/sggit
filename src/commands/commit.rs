use crate::commands::command::{Command, CommandError};
use crate::parsers::parse_repo_config;
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::common_helpers;
use std::path;
use git2::Repository;
use std::process;

pub struct Commit;

fn commit_files(srcs_dsts: &Vec<(&path::Path, &path::Path)>, repo_path: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_path)?;
    let mut index = repo.index()?;

    for src_dst in srcs_dsts {
        index.add_path(src_dst.1)?;
    }

    index.write()?;

    process::Command::new("git").args(["commit","-m","Test"]).current_dir(repo_path).spawn()?;
    Ok(())
}

impl Command for Commit {
    fn run_command(platform_config : Option<&PlatformConfig>) -> Result<(), CommandError> {
        let platform_config = platform_config.expect("No platform setting found. Did you ran `sgit init`?");
        let config = parse_repo_config::RepoConfig::parse_repo_config(platform_config.get_repo_path())?;
        let srcs_dsts = config.get_src_dst_all_files(platform_config.get_platform());

        common_helpers::copy_files(&srcs_dsts, platform_config.get_repo_path())?;
        commit_files(&srcs_dsts, platform_config.get_repo_path())?;
        Ok(())
    }
}
