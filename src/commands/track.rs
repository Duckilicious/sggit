use crate::commands::command::Command;
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config::RepoConfig;
use crate::common_helpers;
use std::env;
use std::path::{Path, PathBuf};

pub struct Track;

pub struct TrackArgs {
    track_path: PathBuf,
    repo_path: PathBuf,
}


impl TrackArgs {
    pub fn new (track_path: &Path, repo_path: &Path) -> Self{
        let track_path = common_helpers::expand_tilde_path(track_path.to_str().unwrap());
        let track_path = env::current_dir()
            .expect("Wasn't able to get current directory")
            .join(track_path);

        if !track_path.exists() {
            panic!("The file you are trying to add doesn't exist {}", track_path.to_str().unwrap());
        }

        TrackArgs {
            track_path, 
            repo_path: common_helpers::expand_tilde_path(repo_path.to_str().unwrap())}
    }
}

impl Command<TrackArgs> for Track {
    fn run_command(
        platform_config: Option<&PlatformConfig>,
        args: Option<TrackArgs>,
    ) {
        let args = args.expect("Argument for command wasn't provided");
        let platform_config = platform_config.expect("Missing platform_config");
        RepoConfig::append_new_file_desc_to_repo_config(
            platform_config,
            args.track_path,
            args.repo_path,
        );
    }
}
