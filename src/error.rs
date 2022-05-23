use log::SetLoggerError;

use crate::format::FormatError;
use thiserror::Error;

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
    #[error("Failed to load {0} config Error {0}")]
    ConfigError(String, String),
}

impl From<SetLoggerError> for Error {
    fn from(e: SetLoggerError) -> Self {
        Error::SetLoggerError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<FormatError> for Error {
    fn from(e: FormatError) -> Self {
        Error::FormatGeneration(e)
    }
}
