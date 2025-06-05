use argh::FromArgs;
use anyhow::Result;
use std::path::PathBuf;

mod commands;
mod config;

use commands::*;

#[derive(FromArgs)]
#[argh(description = "Scatter Gather Git - manage files scattered across the operating system")]
struct Sggit {
    #[argh(subcommand)]
    command: Commands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Commands {
    Init(InitArgs),
    Add(AddArgs),
    Update(UpdateArgs),
    Sync(SyncArgs),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "init")]
#[argh(description = "create an empty git repository")]
struct InitArgs {}

#[derive(FromArgs)]
#[argh(subcommand, name = "add")]
#[argh(description = "add a file to be tracked with remote location")]
struct AddArgs {
    #[argh(positional, description = "remote file path to track")]
    remote_path: PathBuf,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "update")]
#[argh(description = "copy current platform files to local sggit repository")]
struct UpdateArgs {}

#[derive(FromArgs)]
#[argh(subcommand, name = "sync")]
#[argh(description = "update remote location files with files from sggit directory")]
struct SyncArgs {}

fn main() -> Result<()> {
    let args: Sggit = argh::from_env();
    
    match args.command {
        Commands::Init(_) => init_command(),
        Commands::Add(args) => add_command(args.remote_path),
        Commands::Update(_) => update_command(),
        Commands::Sync(_) => sync_command(),
    }
}