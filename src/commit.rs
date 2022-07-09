use crate::command::{Command, CommandError};
use crate::parse_repo_config;
use crate::parse_platform_setting::PlatformConfig;
use std::path;
use git2::Repository;
use std::process;

pub struct Commit;

fn create_diectory_for_dst(dst: &path::Path, repo_path: &path::Path) -> Result<(), CommandError> {
        let root = path::Path::new("/");
        let mut dst_iter = dst;

        dbg!(dst_iter);
        while dst_iter.parent() != None {
            let parent = dst.parent().unwrap();
            if *parent == *dst_iter {
                break;
            }

            dst_iter = parent;
        }

        if *dst_iter == *root {
                return Err(CommandError::from("sgit files path can only use relative paths".to_string()));
        }

        let dst = repo_path.join(dst.parent().unwrap());
        dbg!(&dst);
        let res = std::fs::create_dir_all(&dst);
        if let Err(err) = res {
            return Err(CommandError::from(err))
        }

        Ok(())
}

fn copy_files(srcs_dsts: &Vec<(&path::Path, &path::Path)>, repo_path: &path::Path) -> Result<u64, CommandError> {
    for src_dst in srcs_dsts {
        let src = src_dst.0;
        let dst = src_dst.1;
        create_diectory_for_dst(dst, repo_path)?;
        let dst = repo_path.join(dst);
        dbg!(src);
        dbg!(&dst);
        std::fs::copy(src, dst)?;
    }

    Ok(0)
}

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
    fn run_command(platform_config : &PlatformConfig) -> Result<(), CommandError> {
        let config = parse_repo_config::RepoConfig::parse_repo_config()?;
        let srcs_dsts = config.get_src_dst_all_files(platform_config.get_platform());

        copy_files(&srcs_dsts, platform_config.get_repo_path())?;
        //TODO: Take care of errors
        commit_files(&srcs_dsts, platform_config.get_repo_path()).unwrap();

        Ok(())
    }
}
