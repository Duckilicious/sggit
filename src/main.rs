use sgit::commands::command::{Command, CommandError};
use sgit::commands::commit;
use sgit::parsers::parse_platform_setting::PlatformConfig;
use sgit::commands::init;
use sgit::commands::status;
use std::env;

fn main() -> Result<(), CommandError> {
    let platform_setting = PlatformConfig::parse_platform_config();
    //TODO: Sync, Push, Diff, Clean
    //TODO: Add git remote url to platform setting
    //TODO: Integrate clap - Error if no subcommand
    //TODO: Improve error messages probably remove errors
    //TODO: Add 'sgit add' also with platform option
    //TODO: Add git proxy
    //TODO: Add show tracked files "sgit show"
    let cmd = clap::Command::new(env!("CARGO_CRATE_NAME"))
        .subcommand(clap::Command::new("commit"))
        .subcommand(clap::Command::new("init"))
        .subcommand(clap::Command::new("status"));
    let matches = cmd.get_matches();
    let subcommand = matches.subcommand();
    //TODO swap with exhaustive search
    if let Some(("commit", _cmd)) = subcommand {
        commit::Commit::run_command(platform_setting.ok().as_ref())?;
    } else if let Some(("init", _cmd)) = subcommand {
        init::Init::run_command(platform_setting.ok().as_ref())?;
    } else if let Some(("status", _cmd)) = subcommand {
        status::Status::run_command(platform_setting.ok().as_ref())?;
    }

    Ok(())
}
