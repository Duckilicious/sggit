use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::time::Duration;
use std::thread;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use argh::FromArgs;
use git2::Repository;
use tempfile::NamedTempFile;
use std::os::unix::fs::PermissionsExt;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use std::path::Path;

#[derive(FromArgs, Debug)]
/// A tool to manage configuration files using Git.
struct Cli {
    #[argh(subcommand)]
    command: Commands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
/// Available commands
enum Commands {
    Save(SaveArgs),
    Push(PushArgs),
    Pull(PullArgs),
    Install(InstallArgs),
    Backup(BackupCmd),
    Restore(RestoreCmd),
    Daemon(DaemonCmd),
    Config(ConfigCmd),
}

#[derive(FromArgs, Debug)]
/// Save and commit changes.
#[argh(subcommand, name = "save")]
struct SaveArgs {}

#[derive(FromArgs, Debug)]
/// Push changes to the remote repository.
#[argh(subcommand, name = "push")]
struct PushArgs {}

#[derive(FromArgs, Debug)]
/// Pull changes from the remote repository.
#[argh(subcommand, name = "pull")]
struct PullArgs {}

#[derive(FromArgs, Debug)]
/// Run the installation script and copy files.
#[argh(subcommand, name = "install")]
struct InstallArgs {}

#[derive(FromArgs, PartialEq, Debug)]
/// Backup command
#[argh(subcommand, name = "backup")]
struct BackupCmd {
    #[argh(positional, description = "path to the configuration file")]
    config: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Restore command
#[argh(subcommand, name = "restore")]
struct RestoreCmd {
    #[argh(positional, description = "path to the configuration file")]
    config: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Daemon command
#[argh(subcommand, name = "daemon")]
struct DaemonCmd {
    #[argh(positional, description = "path to the configuration file")]
    config: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Configuration command
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
/// Initialize configuration
#[argh(subcommand, name = "init")]
struct ConfigInit {
    #[argh(positional, description = "path to initialize the configuration")]
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Add configuration
#[argh(subcommand, name = "add")]
struct ConfigAdd {
    #[argh(positional, description = "repository path")]
    repo_path: String,
    #[argh(positional, description = "path to add")]
    path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Remove configuration
#[argh(subcommand, name = "remove")]
struct ConfigRemove {
    #[argh(positional, description = "repository path to remove")]
    repo_path: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// List configurations
#[argh(subcommand, name = "list")]
struct ConfigList {}

#[derive(FromArgs, PartialEq, Debug)]
/// Set prelude script
#[argh(subcommand, name = "set-prelude")]
struct ConfigSetPrelude {
    #[argh(positional, description = "path to the prelude script")]
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
    let uname = ProcessCommand::new("uname").arg("-a").output().expect("Failed to run uname");
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
    ProcessCommand::new("sh")
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
    ProcessCommand::new("git").args(["add", "."]).status().unwrap();
    ProcessCommand::new("git")
        .args(["commit", "-m", &format!("Auto backup: {}", Utc::now())])
        .status()
        .unwrap();
    ProcessCommand::new("git").args(["push"]).status().unwrap();
}

fn _daemon_loop(config: Config, interval_secs: u64) {
    loop {
        println!("Running periodic backup...");
        copy_files(&config, false);
        commit_and_push();
        thread::sleep(Duration::from_secs(interval_secs));
    }
}

fn _load_config(path: &str) -> Config {
    let config_data = fs::read_to_string(path).expect("Failed to read config file");
    serde_json::from_str(&config_data).expect("Invalid config format")
}

fn _save_config(path: &str, config: &Config) {
    let json = serde_json::to_string_pretty(config).unwrap();
    fs::write(path, json).expect("Failed to write config file");
}

#[tokio::main]
async fn main() {
    let cli: Cli = argh::from_env();
    match cli.command {
        Commands::Save(_) => save().await,
        Commands::Push(_) => push().await,
        Commands::Pull(_) => pull().await,
        Commands::Install(_) => install().await,
        Commands::Backup(_) => backup().await,
        Commands::Restore(_) => restore().await,
        Commands::Daemon(_) => daemon().await,
        Commands::Config(_) => config().await,
    }
}

async fn save() {
    println!("Saving changes...");
    let repo = Repository::open(".").expect("Failed to open repository");
    let mut index = repo.index().expect("Failed to get index");
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).expect("Failed to add files to index");
    index.write().expect("Failed to write index");
    let oid = index.write_tree().expect("Failed to write tree");
    let signature = repo.signature().expect("Failed to get signature");
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let tree = repo.find_tree(oid).expect("Failed to find tree");
    repo.commit(Some("HEAD"), &signature, &signature, "Auto backup", &tree, &[&parent_commit]).expect("Failed to commit");
    println!("Changes saved and committed.");
}

async fn push() {
    println!("Pushing changes...");
    let repo = Repository::open(".").expect("Failed to open repository");
    let mut remote = repo.find_remote("origin").expect("Failed to find remote");
    remote.push(&["refs/heads/main:refs/heads/main"], None).expect("Failed to push changes");
    println!("Changes pushed to remote.");
}

async fn pull() {
    println!("Pulling changes...");
    let repo = Repository::open(".").expect("Failed to open repository");
    let mut remote = repo.find_remote("origin").expect("Failed to find remote");
    remote.fetch(&["main"], None, None).expect("Failed to fetch changes");

    let fetch_head = repo.find_reference("FETCH_HEAD").expect("Failed to find FETCH_HEAD");
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).expect("Failed to get commit");

    let analysis = repo.merge_analysis(&[&fetch_commit]).expect("Failed to analyze merge");

    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", "main");
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                r.set_target(fetch_commit.id(), "Fast-Forward").expect("Failed to set target");
                repo.set_head(&refname).expect("Failed to set head");
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force())).expect("Failed to checkout head");
            },
            Err(_) => {
                repo.reference(&refname, fetch_commit.id(), true, "Creating reference").expect("Failed to create reference");
                repo.set_head(&refname).expect("Failed to set head");
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force())).expect("Failed to checkout head");
            }
        };
    } else if analysis.0.is_normal() {
        println!("Normal merge required");
        // Implement normal merge logic if needed
    }

    println!("Pull completed.");
}

async fn install() {
    println!("Running installation script...");
    // Example: Run a prelude script if specified in the config
    let config = _load_config("config.json");
    if let Some(prelude) = &config.prelude {
        _run_prelude(prelude);
    }
    // Copy files to their correct locations
    _copy_files(&config, false);
    println!("Installation completed.");
}

async fn backup() {
    println!("Backing up files...");
    let config = _load_config("config.json");
    _copy_files(&config, false);
    _commit_and_push();
    println!("Backup completed.");
}

async fn restore() {
    println!("Restoring files...");
    pull().await;
    let config = _load_config("config.json");
    _copy_files(&config, true);
    println!("Restore completed.");
}

async fn daemon() {
    println!("Starting daemon...");
    let config = _load_config("config.json");
    tokio::spawn(async move {
        loop {
            backup().await;
            tokio::time::sleep(Duration::from_secs(3600)).await;
        }
    });
}

async fn config() {
    println!("Managing configuration...");
    // Example: Initialize a new configuration
    let new_config = Config {
        platform: "auto".into(),
        prelude: None,
        mappings: HashMap::new(),
    };
    _save_config("config.json", &new_config);
    println!("Configuration initialized.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, NamedTempFile};
    use std::io::Write;

    #[test]
    fn test_add_and_remove_mapping_for_platform() {
        let repo_path = "dotfiles/.vimrc".to_string();
        let file_path = "/home/test/.vimrc".to_string();
        let mut config = Config {
            platform: "arch".to_string(),
            prelude: None,
            mappings: HashMap::new(),
        };

        let entry = PlatformPaths {
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

        copy_files(&config, false); // Should not panic or crash
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

    #[test]
    fn test_run_prelude_executes_script() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "#!/bin/sh\necho hello > {}", file.path().with_extension("out").display()).unwrap();
        let path = file.path();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();

        run_prelude(path.to_str().unwrap());

        let out_path = path.with_extension("out");
        assert!(out_path.exists());
        let contents = std::fs::read_to_string(out_path).unwrap();
        assert!(contents.contains("hello"));
    }

    #[test]
    fn test_commit_and_push_simulation() {
        // NOTE: This test doesn't mock Git, it only ensures the function runs without crashing
        // To properly test this, consider using a wrapper interface with dependency injection
        let result = std::panic::catch_unwind(|| {
            commit_and_push();
        });
        assert!(result.is_ok());
    }
}