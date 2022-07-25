use crate::commands::command::CommandError;
use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config::RepoConfig;
use lazy_static::lazy_static;
use std::path;

pub const REPO_CONFIG_FILE: &str = "repo_config.json";
pub const SGIT_CONFIG_NAME: &str = ".sgit.json";

lazy_static! {
    pub static ref SGIT_PATH: String = std::env::var("HOME").unwrap() + "/.sgit.json";
}

fn create_diectory_for_dst(dst: &path::Path, repo_path: &path::Path) -> Result<(), CommandError> {
    let root = path::Path::new("/");
    let mut dst_iter = dst;

    //      dbg!(dst_iter);
    while dst_iter.parent() != None {
        let parent = dst.parent().unwrap();
        if *parent == *dst_iter {
            break;
        }

        dst_iter = parent;
    }

    if *dst_iter == *root {
        return Err(CommandError::from(
            "sgit files path can only use relative paths".to_string(),
        ));
    }

    let dst = repo_path.join(dst.parent().unwrap());
    //       dbg!(&dst);
    let res = std::fs::create_dir_all(&dst);
    if let Err(err) = res {
        return Err(CommandError::from(err));
    }

    Ok(())
}

fn src_dst_to_dst_src<'a>(
    srcs_dsts: Vec<(&'a path::Path, &'a path::Path)>,
) -> Vec<(&'a path::Path, &'a path::Path)> {
    let mut dsts_srcs = vec![];
    for src_dst in srcs_dsts {
        let dst_src: (&path::Path, &path::Path) = (src_dst.1, src_dst.0);
        dsts_srcs.push(dst_src);
    }

    dsts_srcs
}

//TODO: Dedup copy_files_to_repo and copy_files_from_repo
pub fn copy_files_to_repo(
    platform_config: &PlatformConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_config = RepoConfig::parse_repo_config(platform_config.get_repo_path())?;
    let srcs_dsts = repo_config.get_src_dst_all_files(platform_config.get_platform());
    for src_dst in srcs_dsts {
        let src = src_dst.0;
        let dst = src_dst.1;

        create_diectory_for_dst(dst, platform_config.get_repo_path())?;
        let dst = platform_config.get_repo_path().join(dst);
        // Skip copying files in the repo
        if *src == *dst {
            continue;
        }

        //      dbg!(src);
        //      dbg!(&dst);
        std::fs::copy(src, dst)?;
    }

    Ok(())
}

pub fn copy_files_from_repo(platform_config: &PlatformConfig) {
    let repo_config = RepoConfig::parse_repo_config(platform_config.get_repo_path())
        .expect("Unable to parse repo config");
    let srcs_dsts = repo_config.get_src_dst_all_files(platform_config.get_platform());
    let dsts_srcs = src_dst_to_dst_src(srcs_dsts);
    for src_dst in dsts_srcs {
        let src = src_dst.0;
        let dst = src_dst.1;

        create_diectory_for_dst(dst, platform_config.get_repo_path())
            .expect("Failed to create directory");
        let src = platform_config.get_repo_path().join(src);
        // Skip copying files in the repo
        if *src == *dst {
            continue;
        }

        std::fs::copy(&src, &dst).expect(
            format!(
                "Failed to copy files {} {}
                \n Did you commit your changes?",
                src.to_str().expect("Failed to parse file name"),
                dst.to_str().expect("Failed to parse file name")
            )
            .as_str(),
        );
    }
}
