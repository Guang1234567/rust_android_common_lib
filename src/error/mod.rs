use std::convert::From;
use std::error::Error;
use std::io::Error as IoError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::mpsc::{RecvError as MpscRecvError, SendError as MpscSendError};

use diesel::ConnectionError;
use diesel::migration::RunMigrationsError;
use diesel::result::Error as CrudError;
use dotenv::Error as DotEnvError;
use jni::errors::Error as JniError;
use log::SetLoggerError;

use std::marker::Send;

use LibError::*;

#[derive(Debug)]
pub enum LibError {
    EmptyVec,
    DotEnvErr(Box<dyn Error>),
    LogErr(Box<dyn Error>),
    DatabaseErr(Box<dyn Error>),
    JniErr(Box<dyn Error>),
    MpscErr(Box<dyn Error>),
    IoErr(Box<dyn Error>),
}

pub type LibResult<T> = Result<T, LibError>;

impl From<SetLoggerError> for LibError {
    fn from(err: SetLoggerError) -> Self {
        LogErr(err.into())
    }
}

impl From<JniError> for LibError {
    fn from(err: JniError) -> Self {
        JniErr(err.into())
    }
}

impl From<DotEnvError> for LibError {
    fn from(err: DotEnvError) -> Self {
        DotEnvErr(err.into())
    }
}

impl From<MpscRecvError> for LibError {
    fn from(err: MpscRecvError) -> Self {
        MpscErr(err.into())
    }
}

impl<T> From<MpscSendError<T>> for LibError
    where T: Send + 'static {
    fn from(err: MpscSendError<T>) -> Self {
        MpscErr(err.into())
    }
}

impl From<IoError> for LibError {
    fn from(err: IoError) -> Self {
        IoErr(err.into())
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
            JniErr(e) => e.fmt(f),
            MpscErr(e) => e.fmt(f),
            IoErr(e) => e.fmt(f),
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
            JniErr(e) => e.description(),
            MpscErr(e) => e.description(),
            IoErr(e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            EmptyVec => None,
            DotEnvErr(e) => Some(&**e),
            LogErr(e) => Some(&**e),
            DatabaseErr(e) => Some(&**e),
            JniErr(e) => Some(&**e),
            MpscErr(e) => Some(&**e),
            IoErr(e) => Some(&**e),
        }
    }
}