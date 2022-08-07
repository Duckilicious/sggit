use crate::commands::command::Command;
use crate::common_helpers;
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config;
use git2::Repository;
use std::path;
use std::process;

pub struct Commit;

pub struct CommitArgs<'a> {
    msg: &'a str 
}

impl<'a> CommitArgs<'a> {
    pub fn new(msg: &'a str) -> Self {
        CommitArgs { msg }
    }
}
fn commit_files(srcs_dsts: &Vec<(&path::Path, &path::Path)>, repo_path: &path::Path, msg: &str) {
    let repo = Repository::open(repo_path)
        .unwrap_or_else(|err| panic!("Failed to find git repositorty in {}", err));
    let mut index = repo.index().expect("Failed to get index file of the repo");

    for src_dst in srcs_dsts {
        index.add_path(src_dst.1).unwrap_or_else(|err| {
            panic!(
                "Failed to add path {}, {}, {}",
                src_dst.0.to_string_lossy(),
                src_dst.1.to_string_lossy(),
                err
            )
        });
    }

    index.write().expect("Unable to add files to repo");

    process::Command::new("git")
        .args(["commit", "-m", msg])
        .current_dir(repo_path)
        .spawn()
        .expect("Failed to commit files to repo");
}

impl Command<CommitArgs<'_>> for Commit {
    fn run_command(platform_config: Option<&PlatformConfig>, args: Option<CommitArgs>) {
        let platform_config =
            platform_config.expect("No platform setting found. Did you ran `sgit init`?");
        common_helpers::copy_files_to_repo(platform_config);

        let config =
            parse_repo_config::RepoConfig::parse_repo_config(platform_config.get_repo_path());
        let srcs_dsts = config.get_src_dst_all_files(platform_config.get_platform());
        commit_files(&srcs_dsts, platform_config.get_repo_path(), args.unwrap().msg);
    }
}
