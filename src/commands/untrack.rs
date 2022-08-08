use crate::commands::command::Command;
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config::RepoConfig;
use crate::common_helpers;
use std::path::{Path, PathBuf};

pub struct Untrack;
pub struct UntrackArgs {
    repo_path: PathBuf,
}

impl UntrackArgs {
    pub fn new (repo_path: &Path) -> Self {
        UntrackArgs {
            repo_path: common_helpers::expand_tilde_path(repo_path.to_str().unwrap())}
        }
}

impl Command<UntrackArgs> for Untrack {
    fn run_command(
        platform_config: Option<&PlatformConfig>,
        args: Option<UntrackArgs>,
    ) {
        let args = args.expect("Argument for command wasn't provided");
        let platform_config = platform_config.expect("Missing platform_config");
        RepoConfig::remove_file_from_repo_config(
            platform_config,
            args.repo_path.as_path(),
        );

        let mut target_path = platform_config.get_repo_path().to_path_buf();
        target_path.push(args.repo_path);
        let _ = std::fs::remove_file(target_path);
    }
}
