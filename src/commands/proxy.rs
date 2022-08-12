use crate::commands::command::Command;
use crate::parsers::parse_platform_setting::PlatformConfig;
use std::io::{self, Write};
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
        let args: Vec<_> = args.command.split(' ').collect();

        let output = process::Command::new("git")
            .args(args)
            .current_dir(platform_config.get_repo_path())
            .output()
            .expect("Failed to run command");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
