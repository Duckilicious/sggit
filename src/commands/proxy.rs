use crate::commands::command::Command;
use crate::parsers::parse_platform_setting::PlatformConfig;
use std::process;

pub struct Proxy;

pub struct ProxyArgs<'a> {
    command: &'a str,
}

impl<'a> ProxyArgs<'a> {
    pub fn new(command: &'a str) -> Self {
        ProxyArgs { command }
    }
}

impl Command<ProxyArgs<'_>> for Proxy {
    fn run_command(platform_config: Option<&PlatformConfig>, args: Option<ProxyArgs>) {
        let args = args.expect("Argument for command wasn't provided");
        let platform_config = platform_config.expect("Missing platform_config");

        process::Command::new("git")
            .args([args.command])
            .current_dir(platform_config.get_repo_path())
            .spawn()
            .unwrap_or_else(|err| {
                panic!("Failed to run provided command {} {}", args.command, err)
            });
    }
}
