use std::fmt::{Display, Formatter};
use log::SetLoggerError;

#[derive(Debug)]
pub enum Error {
    SetLoggerError(SetLoggerError),
    IOError(std::io::Error),
    SerdeJson(serde_json::Error),
}
impl Display for Error{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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
