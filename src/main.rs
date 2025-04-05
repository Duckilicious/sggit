use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::{thread, env};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::process::exit;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// CLI + Daemon to backup/restore config files
struct Cli {
    #[argh(subcommand)]
    command: Commands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Commands {
    Backup(BackupCmd),
    Restore(RestoreCmd),
    Daemon(DaemonCmd),
    Config(ConfigCmd),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Backup files
#[argh(subcommand, name = "backup")]
struct BackupCmd {
    /// path to config file
    #[argh(option)]
    config: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Restore files
#[argh(subcommand, name = "restore")]
struct RestoreCmd {
    /// path to config file
    #[argh(option)]
    config: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Run daemon
#[argh(subcommand, name = "daemon")]
struct DaemonCmd {
    /// path to config file
    #[argh(option)]
    config: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Config management
#[argh(subcommand, name = "config")]
struct ConfigCmd {
    #[argh(subcommand)]
    action: ConfigActions,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum ConfigActions {
    Init(ConfigInit),
    Add(ConfigAdd),
    Remove(ConfigRemove),
    List(ConfigList),
    SetPrelude(ConfigSetPrelude),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Initialize config
#[argh(subcommand, name = "init")]
struct ConfigInit {
    /// config file path
    #[argh(option, default = "String::from(\"config.json\")")]
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Add a file mapping
#[argh(subcommand, name = "add")]
struct ConfigAdd {
    /// path in the repo
    #[argh(positional)]
    repo_path: String,
    /// real path on disk
    #[argh(positional)]
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Remove a file mapping
#[argh(subcommand, name = "remove")]
struct ConfigRemove {
    /// repo path to remove
    #[argh(positional)]
    repo_path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// List mappings
#[argh(subcommand, name = "list")]
struct ConfigList {}

#[derive(FromArgs, PartialEq, Debug)]
/// Set prelude script
#[argh(subcommand, name = "set-prelude")]
struct ConfigSetPrelude {
    /// path to prelude script
    #[argh(positional)]
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    platform: String,
    prelude: Option<String>,
    mappings: HashMap<String, PlatformPaths>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlatformPaths {
    default: Option<String>,
    arch: Option<String>,
    ubuntu: Option<String>,
    wsl: Option<String>,
}

fn detect_platform() -> String {
    let uname = Command::new("uname").arg("-a").output().expect("Failed to run uname");
    let output = String::from_utf8_lossy(&uname.stdout).to_lowercase();
    if output.contains("microsoft") {
        "wsl".to_string()
    } else if output.contains("arch") {
        "arch".to_string()
    } else if output.contains("ubuntu") {
        "ubuntu".to_string()
    } else {
        "default".to_string()
    }
}

fn run_prelude(prelude_path: &str) {
    println!("Running prelude script: {}", prelude_path);
    Command::new("sh")
        .arg(prelude_path)
        .status()
        .expect("Failed to run prelude script");
}

fn copy_files(config: &Config, reverse: bool) {
    let platform = detect_platform();

    for (repo_path, paths) in &config.mappings {
        let platform_path = match platform.as_str() {
            "arch" => &paths.arch,
            "ubuntu" => &paths.ubuntu,
            "wsl" => &paths.wsl,
            _ => &paths.default,
        };

        if let Some(target_path) = platform_path {
            let from = if reverse {
                PathBuf::from(repo_path)
            } else {
                PathBuf::from(target_path)
            };
            let to = if reverse {
                PathBuf::from(target_path)
            } else {
                PathBuf::from(repo_path)
            };

            if from.exists() {
                fs::create_dir_all(to.parent().unwrap()).ok();
                fs::copy(&from, &to).expect("Failed to copy file");
                println!("Copied {:?} to {:?}", from, to);
            }
        }
    }
}

fn commit_and_push() {
    Command::new("git").args(["add", "."]).status().unwrap();
    Command::new("git")
        .args(["commit", "-m", &format!("Auto backup: {}", Utc::now())])
        .status()
        .unwrap();
    Command::new("git").args(["push"]).status().unwrap();
}

fn daemon_loop(config: Config, interval_secs: u64) {
    loop {
        println!("Running periodic backup...");
        copy_files(&config, false);
        commit_and_push();
        thread::sleep(Duration::from_secs(interval_secs));
    }
}

fn load_config(path: &str) -> Config {
    let config_data = fs::read_to_string(path).expect("Failed to read config file");
    serde_json::from_str(&config_data).expect("Invalid config format")
}

fn save_config(path: &str, config: &Config) {
    let json = serde_json::to_string_pretty(config).unwrap();
    fs::write(path, json).expect("Failed to write config file");
}

fn main() {
    let cli: Cli = argh::from_env();

    match cli.command {
        Commands::Backup(BackupCmd { config }) => {
            let config = load_config(&config);
            if let Some(prelude) = &config.prelude {
                run_prelude(prelude);
            }
            copy_files(&config, false);
        }
        Commands::Restore(RestoreCmd { config }) => {
            let config = load_config(&config);
            if let Some(prelude) = &config.prelude {
                run_prelude(prelude);
            }
            copy_files(&config, true);
        }
        Commands::Daemon(DaemonCmd { config }) => {
            let config = load_config(&config);
            if let Some(prelude) = &config.prelude {
                run_prelude(prelude);
            }
            daemon_loop(config, 3600);
        }
        Commands::Config(ConfigCmd { action }) => {
            match action {
                ConfigActions::Init(ConfigInit { path }) => {
                    let new_config = Config {
                        platform: "auto".into(),
                        prelude: None,
                        mappings: HashMap::new(),
                    };
                    save_config(&path, &new_config);
                    println!("Initialized new config at {}", path);
                }
                ConfigActions::Add(ConfigAdd { repo_path, path }) => {
                    let mut config = load_config("config.json");
                    let platform = detect_platform();

                    let mut entry = config
                        .mappings
                        .remove(&repo_path)
                        .unwrap_or(PlatformPaths {
                            default: None,
                            arch: None,
                            ubuntu: None,
                            wsl: None,
                        });

                    match platform.as_str() {
                        "arch" => entry.arch = Some(path),
                        "ubuntu" => entry.ubuntu = Some(path),
                        "wsl" => entry.wsl = Some(path),
                        _ => entry.default = Some(path),
                    }

                    config.mappings.insert(repo_path.clone(), entry);
                    save_config("config.json", &config);
                    println!("Mapping updated for platform '{}'", platform);
                }
                ConfigActions::Remove(ConfigRemove { repo_path }) => {
                    let mut config = load_config("config.json");
                    config.mappings.remove(&repo_path);
                    save_config("config.json", &config);
                    println!("Removed mapping for {}", repo_path);
                }
                ConfigActions::List(_) => {
                    let config = load_config("config.json");
                    println!("Current mappings:");
                    for (k, v) in config.mappings {
                        println!("{} => {:?}", k, v);
                    }
                }
                ConfigActions::SetPrelude(ConfigSetPrelude { path }) => {
                    let mut config = load_config("config.json");
                    config.prelude = Some(path);
                    save_config("config.json", &config);
                    println!("Prelude script set.");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_add_and_remove_mapping_for_platform() {
        let repo_path = "dotfiles/.vimrc".to_string();
        let file_path = "/home/test/.vimrc".to_string();
        let mut config = Config {
            platform: "arch".to_string(),
            prelude: None,
            mappings: HashMap::new(),
        };

        let mut entry = PlatformPaths {
            default: None,
            arch: Some(file_path.clone()),
            ubuntu: None,
            wsl: None,
        };

        config.mappings.insert(repo_path.clone(), entry);
        assert!(config.mappings.contains_key(&repo_path));
        assert_eq!(config.mappings[&repo_path].arch.as_ref().unwrap(), &file_path);

        config.mappings.remove(&repo_path);
        assert!(!config.mappings.contains_key(&repo_path));
    }

    #[test]
    fn test_copy_files_skips_missing_paths() {
        let config = Config {
            platform: detect_platform(),
            prelude: None,
            mappings: HashMap::from([(
                "some/path".to_string(),
                PlatformPaths {
                    default: None,
                    arch: None,
                    ubuntu: None,
                    wsl: None,
                },
            )]),
        };

        // Should not panic or crash
        copy_files(&config, false);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            platform: "auto".into(),
            prelude: Some("setup.sh".into()),
            mappings: HashMap::new(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.platform, "auto");
        assert_eq!(parsed.prelude, Some("setup.sh".into()));
    }
}

