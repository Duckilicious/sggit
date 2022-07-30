use crate::commands::command::{Command, CommandError, NoArgs};
use crate::common_helpers;
use crate::parsers::parse_platform_setting::PlatformConfig;
pub struct Sync;

impl Command<NoArgs> for Sync {
    fn run_command(platform_config: Option<&PlatformConfig>, _: Option<NoArgs>) -> Result<(), CommandError> {
        common_helpers::copy_files_from_repo(platform_config.expect("No platform_config"));
        Ok(())
    }
}
