use crate::parsers::parse_repo_config::ParseError;
use crate::parsers::parse_platform_setting::PlatformConfig;
use std::error::Error;
use std::fmt;

pub mod init;
pub mod commit;

#[derive(Debug)]
pub struct CommandError {
    err : CommandErrorImpl,
}

impl Error for CommandError {
    fn description(&self) -> &str {
        ""
    }
}

#[derive(Debug)]
enum CommandErrorImpl {
    Parse(ParseError),
    CommitError(String),
    CommitCopyError(std::io::Error),
    LazyError(Box<dyn std::error::Error>),
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

impl From<Box<dyn std::error::Error>> for CommandError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        CommandError { err: CommandErrorImpl::LazyError(err)}
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.err {
            CommandErrorImpl::Parse(err) => println!("Command Failed {}", err),
            CommandErrorImpl::UnknownCommand => println!("UnknownCommand"),
            CommandErrorImpl::CommitError(err) => println!("{}", err),
            CommandErrorImpl::CommitCopyError(err) => println!("{}", err),
            CommandErrorImpl::LazyError(err) => println!("{}", err),
        };
        Ok(())
    }
}

impl CommandError {
    pub fn new() -> Self {
        CommandError {err: CommandErrorImpl::UnknownCommand }
    }
}

pub trait Command {
    fn run_command(platform_config: Option<&PlatformConfig>) -> Result<(), CommandError>;
}
