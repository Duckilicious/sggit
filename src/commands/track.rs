use crate::commands::command::Command;
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config::RepoConfig;
use std::env;
use std::path::PathBuf;

pub struct Track;
pub struct TrackArgs {
    path: PathBuf,
    repo_path: PathBuf,
}

impl TrackArgs {
    pub fn new (path: PathBuf, repo_path: PathBuf) -> Self{
        TrackArgs {path, repo_path}
    }
}

impl Command<TrackArgs> for Track {
    fn run_command(
        platform_config: Option<&PlatformConfig>,
        args: Option<TrackArgs>,
    ) {
        let args = args.expect("Argument for command wasn't provided");
        let platform_config = platform_config.expect("Missing platform_config");
        let target_file = env::current_dir()
            .expect("Wasn't able to get current directory")
            .join(args.path);
        RepoConfig::append_new_file_desc_to_repo_config(
            platform_config,
            target_file,
            args.repo_path,
        );
    }
}
