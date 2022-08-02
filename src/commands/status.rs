use crate::commands::command::{Command, NoArgs};
use crate::common_helpers;
use crate::parsers::parse_platform_setting::PlatformConfig;
use std::process;
use std::path;
pub struct Status;

#[derive(Debug)]
pub struct TempKeepRepoConfig<'a> {
    repo_path: &'a path::Path,
}

impl<'a> TempKeepRepoConfig<'a> {
    fn new(repo_path: &'a path::Path) -> Self {
        let err_msg = "Failed to temporary commit the repo_config";
        process::Command::new("git")
            .args(["add", "repo_config.json"])
            .current_dir(repo_path)
            .output()
            .expect(err_msg);
        process::Command::new("git")
            .args(["commit", "--allow-empty", "-m", "temp"])
            .current_dir(repo_path)
            .output()
            .expect(err_msg);

        TempKeepRepoConfig { repo_path }
    }
}

impl<'a> Drop for TempKeepRepoConfig<'a> {
    fn drop(&mut self) {
        process::Command::new("git")
            .args(["reset", "HEAD~"])
            .current_dir(self.repo_path)
            .output()
            .unwrap();
    }
}

impl Command<NoArgs> for Status {
    fn run_command(platform_config: Option<&PlatformConfig>, _ :Option<NoArgs>) {
        let platform_config = platform_config.unwrap_or_else(|| {
            panic!("Missing platform config, try to run `sgit init`")
        });

        common_helpers::copy_files_to_repo(platform_config);
        let mut status_proc = process::Command::new("git")
            .args(["status"])
            .current_dir(platform_config.get_repo_path())
            .spawn()
            .unwrap();
        status_proc.wait().expect("Failed to produce git status");

        let _delay_soft_reset = TempKeepRepoConfig::new(platform_config.get_repo_path());
        process::Command::new("git")
            .args(["reset", "--hard", "HEAD"])
            .current_dir(platform_config.get_repo_path())
            .output()
            .expect("Failed to reset branch after status");
        process::Command::new("git")
            .args(["clean", "-fd"])
            .current_dir(platform_config.get_repo_path())
            .output()
            .expect("Unable to run git clean");
    }
}
