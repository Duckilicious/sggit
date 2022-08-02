use crate::parsers::parse_platform_setting::PlatformConfig;

pub struct NoArgs;
pub trait Command<T> {
    fn run_command(platform_config: Option<&PlatformConfig>, args: Option<T>);
}
