use std::env;
use vcfg::parse_platform_setting::PlatformConfig;
use vcfg::command::{Command, CommandError};
use vcfg::commit;

fn main() -> Result<(), CommandError> {
    let args: Vec<String> = env::args().collect();
    let platform_setting = PlatformConfig::parse_platform_config().unwrap();

    //TODO: Integrate clap
    match args[1].as_str() {
        //TODO: Sync, Push, Init, Status, Diff
        "commit" => commit::Commit::run_command(&platform_setting),
        _ => {
            println!("Unknown command");
            Err(CommandError::new())
        }
    }
}
