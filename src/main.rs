use sgit::commands::command::{Command, CommandError};
use sgit::commands::commit;
use sgit::commands::init;
use sgit::commands::status;
use sgit::commands::sync;
use sgit::commands::add;
use sgit::commands::add::AddArgs;
use sgit::parsers::parse_platform_setting::PlatformConfig;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "sgit")]
#[clap(about = "Scatter-Gather git - sgit\n Tracking scattered files made easy", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

// TODO: Move enum to commands.rs restructure dedup structs with enums with the same name.
#[derive(Debug, Subcommand)]
enum Commands {
    /// Clone and set up your remote repo that is managed with sgit
    #[clap(arg_required_else_help = true)]
    Clone {
        /// The remote to clone
        remote: String,
    },
    /// Push to your remote sgit managed repo
    #[clap(arg_required_else_help = true)]
    Push {
        /// The remote name to target
        remote: String,
    },
    /// Add files to track to your sgit managed repo
    #[clap(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[clap(required = true)]
        path: PathBuf,
        repo_path: PathBuf,
    },

    /// Commit all changes made in your scattered files
    Commit,

    /// Init a sgit - It will track it's own config
    Init,

    /// Show your sgit managed repo status
    Status,

    /// Copy the current commited version of your files to their original location in your platform
    Sync,

    /// Print all tracked files
    Show,

    /// Print the git diff of tracked files
    Diff,

    /// Clean sgit repo from untracked files
    Clean,
}

fn main() -> Result<(), CommandError> {
    let platform_setting = PlatformConfig::parse_platform_config().unwrap();
    //TODO: Add git remote url to platform setting
    //TODO: Integrate clap - Error if no subcommand
    //TODO: Replace errors with panics and messages
    //TODO: Add 'sgit add' also with platform option
    //TODO: Add git proxy

    dbg!(&platform_setting);
    let _num: u64 = 1000000000;
    let args = Cli::parse();

    match args.command {
        Commands::Commit => {
            commit::Commit::run_command(Some(&platform_setting), None)?;
        }
        Commands::Init => {
            init::Init::run_command(Some(&platform_setting), None)?;
        }
        Commands::Status => {
            status::Status::run_command(Some(&platform_setting), None)?;
        }
        Commands::Sync => {
            sync::Sync::run_command(Some(&platform_setting), None)?;
        }
        Commands::Clone{remote: _} => {
            std::todo!();
        }
        Commands::Push{remote: _}=> {
            std::todo!();
        }
        Commands::Add{path, repo_path} => {
            add::Add::run_command(Some(&platform_setting), Some(AddArgs::new(path, repo_path)))?;
        }
        Commands::Show => {
            std::todo!();
        }
        Commands::Diff => {
            std::todo!();
        }
        Commands::Clean => {
            std::todo!();
        }
    }

    Ok(())
}
