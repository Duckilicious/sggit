use crate::commands::command::Command;
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config::RepoConfig;
use std::path::Path;

pub struct Untrack;
pub struct UntrackArgs<'a> {
    repo_path: &'a Path,
}

impl<'a> UntrackArgs<'a> {
    pub fn new (repo_path: &'a Path) -> Self{
        UntrackArgs {repo_path}
    }
}

impl Command<UntrackArgs<'_>> for Untrack {
    fn run_command(
        platform_config: Option<&PlatformConfig>,
        args: Option<UntrackArgs>,
    ) {
        let args = args.expect("Argument for command add wasn't provided");
        let platform_config = platform_config.expect("Missing platform_config");
        RepoConfig::remove_file_from_repo_config(
            platform_config,
            args.repo_path,
        );
    }
}
