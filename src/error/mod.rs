use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use diesel::ConnectionError;
use diesel::migration::RunMigrationsError;
use diesel::result::Error as CrudError;
use log::SetLoggerError;
use dotenv::Error as DotEnvError;

use LibError::*;

#[derive(Debug)]
pub enum LibError {
    EmptyVec,
    DotEnvErr(Box<dyn Error>),
    LogErr(Box<dyn Error>),
    DatabaseErr(Box<dyn Error>),
}

pub type LibResult<T> = Result<T, LibError>;

impl From<SetLoggerError> for LibError {
    fn from(err: SetLoggerError) -> Self {
        LogErr(err.into())
    }
}

impl From<DotEnvError> for LibError {
    fn from(err: DotEnvError) -> Self {
        DotEnvErr(err.into())
    }
}

impl From<ConnectionError> for LibError {
    fn from(err: ConnectionError) -> Self {
        DatabaseErr(err.into())
    }
}

impl From<RunMigrationsError> for LibError {
    fn from(err: RunMigrationsError) -> Self {
        DatabaseErr(err.into())
    }
}

impl From<CrudError> for LibError {
    fn from(err: CrudError) -> Self {
        DatabaseErr(err.into())
    }
}

impl Display for LibError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            EmptyVec => write!(f, "please use a vector with at least one element"),
            DotEnvErr(e) => e.fmt(f),
            LogErr(e) => e.fmt(f),
            DatabaseErr(e) => e.fmt(f),
        }
    }
}

impl Error for LibError {
    fn description(&self) -> &str {
        match self {
            EmptyVec => "empty vectors not allowed",
            DotEnvErr(e) => e.description(),
            LogErr(e) => e.description(),
            DatabaseErr(e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            EmptyVec => None,
            DotEnvErr(e) => Some(&**e),
            LogErr(e) => Some(&**e),
            DatabaseErr(e) => Some(&**e),
        }
    }
}