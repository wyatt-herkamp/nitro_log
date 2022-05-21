use log::SetLoggerError;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::format::FormatError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    SetLoggerError(SetLoggerError),
    #[error("{0}")]
    IOError(std::io::Error),
    #[error("{0}")]
    SerdeJson(serde_json::Error),
    #[error("Failed to generate a Format: {0}")]
    FormatGeneration(FormatError),
}

impl From<SetLoggerError> for Error {
    fn from(e: SetLoggerError) -> Self {
        return Error::SetLoggerError(e);
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::SerdeJson(e);
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::IOError(e);
    }
}

impl From<FormatError> for Error {
    fn from(e: FormatError) -> Self {
        return Error::FormatGeneration(e);
    }
}
