use sgit::commands::command::Command;
use sgit::commands::commit::{Commit, CommitArgs};
use sgit::commands::init::Init;
use sgit::commands::status::Status;
use sgit::commands::sync::Sync;
use sgit::commands::track::{Track, TrackArgs};
use sgit::commands::untrack::{Untrack, UntrackArgs};
use sgit::commands::proxy::{Proxy, ProxyArgs};
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

// TODO: Move enum to commands.rs restructure and dedup structs with enums with the same name.
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
        #[clap(required = true)]
        remote: String,
    },
    /// Add files to track to your sgit managed repo
    #[clap(arg_required_else_help = true)]
    Track {
        /// Stuff to add
        #[clap(required = true)]
        path: PathBuf,
        repo_path: PathBuf,
    },

    /// Remove tracked files from your sgit managed repo
    #[clap(arg_required_else_help = true)]
    Untrack {
        /// Stuff to add
        #[clap(required = true)]
        repo_path: PathBuf,
    },

    /// Commit all changes made in your scattered files
    #[clap(arg_required_else_help = true)]
    Commit {
        /// Commit message
        #[clap(required = true, short = 'm')]
        msg: String
    },

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

    #[clap(arg_required_else_help = true)]
    Proxy {
        #[clap(required = true, short = 'c')]
        command: String
    }
        ,
}

fn main() {
    let platform_setting = PlatformConfig::parse_platform_config();
    //TODO: Add git remote url to platform setting
    //TODO: Add 'sgit track' platform option
    //TODO: Add 'sgit untrack' platform option (maybe `sgit untrack platform ...`)
    //TODO: Add git proxy
    //TODO: Change init to get command line args instead of prompting

    let _num: u64 = 1000000000;
    let args = Cli::parse();

    match args.command {
        Commands::Commit{msg} => {
            Commit::run_command(platform_setting.as_ref(), Some(CommitArgs::new(&msg)));
        }
        Commands::Init => {
            Init::run_command(platform_setting.as_ref(), None);
        }
        Commands::Status => {
            Status::run_command(platform_setting.as_ref(), None);
        }
        Commands::Sync => {
            Sync::run_command(platform_setting.as_ref(), None);
        }
        Commands::Clone{remote: _} => {
            std::todo!();
        }
        Commands::Push{remote: _}=> {
            std::todo!();
        }
        Commands::Track{path, repo_path} => {
            Track::run_command(platform_setting.as_ref(), Some(TrackArgs::new(path, repo_path)));
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
        Commands::Untrack{repo_path} => {
            Untrack::run_command(platform_setting.as_ref(), Some(UntrackArgs::new(repo_path.as_path())));
        }
        Commands::Proxy{command} => {
            Proxy::run_command(platform_setting.as_ref(), Some(ProxyArgs::new(&command)))
        }
    }
}
