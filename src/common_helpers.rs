use lazy_static::lazy_static;

pub const REPO_CONFIG_FILE: &str = "repo_config.json";
pub const SGIT_CONFIG_NAME: &str = ".sgit.json";

lazy_static! {
    pub static ref SGIT_PATH: String = std::env::var("HOME").unwrap() + "/.sgit.json";
}

