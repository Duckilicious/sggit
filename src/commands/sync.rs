use crate::commands::command::{Command, CommandError};
use crate::common_helpers;
use crate::parsers::parse_platform_setting::PlatformConfig;
pub struct Sync;

impl Command for Sync {
    fn run_command(platform_config: Option<&PlatformConfig>) -> Result<(), CommandError> {
        common_helpers::copy_files_from_repo(platform_config.expect("No platform_config"));
        Ok(())
    }
}
