use crate::parse_repo_config::ParseError;
use crate::parse_platform_setting::PlatformConfig;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CommandError {
    err : CommandErrorImpl,
}

#[derive(Debug)]
enum CommandErrorImpl {
    Parse(ParseError),
    CommitError(String),
    CommitCopyError(std::io::Error),
    UnknownCommand,
}

impl From<String> for CommandError {
    fn from(err: String) -> Self {
        CommandError { err: CommandErrorImpl::CommitError(err) }
    }
}

impl From<ParseError> for CommandError {
    fn from(err : ParseError) -> Self {
        CommandError { err: CommandErrorImpl::Parse(err) }
    }
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        CommandError { err: CommandErrorImpl::CommitCopyError(err) }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.err {
            CommandErrorImpl::Parse(err) => println!("Command Failed {}", err),
            CommandErrorImpl::UnknownCommand => println!("UnknownCommand"),
            CommandErrorImpl::CommitError(err) => println!("{}", err),
            CommandErrorImpl::CommitCopyError(err) => println!("{}", err),
        };
        Ok(())
    }
}

impl CommandError {
    pub fn new() -> Self {
        CommandError {err: CommandErrorImpl::UnknownCommand }
    }
}

impl Error for CommandError {}


pub trait Command {
    fn run_command(platform_config: &PlatformConfig) -> Result<(), CommandError>;
}
