use crate::parsers::parse_platform_setting::PlatformConfig;
use crate::parsers::parse_repo_config::RepoConfig;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

pub const REPO_CONFIG_FILE: &str = "repo_config.json";
pub const SGGIT_CONFIG_NAME: &str = ".sggit.json";

lazy_static! {
    pub static ref SGGIT_PATH: String = std::env::var("HOME").unwrap() + "/.sggit.json";
}

pub fn expand_tilde_path(path: &str) -> PathBuf {
        PathBuf::from(
            String::from_utf8(tilde_expand::tilde_expand(path.trim().as_bytes()))
                .expect("Unable to parse pah into a string")
        )
}

fn create_diectory_for_dst(dst: &Path) {
    if dst.has_root() {
        panic!("sggit files path can only use relative paths {} is absolute", dst.to_string_lossy());
    }

    let res = std::fs::create_dir_all(&dst.parent().expect("Bad path - No parent directory for file in rpeo"));
    if let Err(err) = res {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            panic!("Couldn't create dire {}", err);
        }
    }
}

fn src_dst_to_dst_src<'a>(
    srcs_dsts: Vec<(&'a Path, &'a Path)>,
) -> Vec<(&'a Path, &'a Path)> {
    let mut dsts_srcs = vec![];
    for src_dst in srcs_dsts {
        let dst_src: (&Path, &Path) = (src_dst.1, src_dst.0);
        dsts_srcs.push(dst_src);
    }

    dsts_srcs
}

fn copy_files_to_or_from_repo(platform_config: &PlatformConfig, is_to_repo: bool) {
    let repo_config = RepoConfig::parse_repo_config(platform_config.get_repo_path());
    let mut srcs_dsts = repo_config.get_src_dst_all_files(platform_config.get_platform());
    if !is_to_repo {
        srcs_dsts = src_dst_to_dst_src(srcs_dsts);
    }

    let prep_for_copy = |path: PathBuf| platform_config.get_repo_path().join(path);

    for src_dst in srcs_dsts {
        let mut src = src_dst.0.to_owned();
        let mut dst = src_dst.1.to_owned();

        if is_to_repo {
            dst = prep_for_copy(dst);
        } else {
            src = prep_for_copy(src);
            create_diectory_for_dst(&*dst);
        };


        // Skip copying files in the repo
        if *src == *dst {
            continue;
        }

        std::fs::copy(&src, &dst).unwrap_or_else(|err| {
            panic!(
                "Failed to copy files {} {} {}
                \n Did you add a file to repo without commitng your changes?",
                src.to_str().expect("Failed to parse file name"),
                dst.to_str().expect("Failed to parse file name"),
                err
            )
        });
    }
}

pub fn copy_files_from_repo(platform_config: &PlatformConfig) {
    copy_files_to_or_from_repo(platform_config, false);
}

pub fn copy_files_to_repo(platform_config: &PlatformConfig) {
    copy_files_to_or_from_repo(platform_config, true);
}
