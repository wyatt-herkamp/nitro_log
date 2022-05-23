use std::io::Write;
use log::Record;
use crate::loggers::LoggerTarget;

/// This is a wrapper around the actual Writer provided. Allowing for a consistent write experience through the log call
/// It will automatically return the writer upon drop. Running the post write tasks. Some implementations will not do anything.
pub struct LoggerWriter<'a> {
    pub internal: Box<dyn Write>,
    pub record: &'a Record<'a>,
    pub logger: Box<&'a  dyn LoggerTarget>,
}


impl<'a> Write for LoggerWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.internal.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.internal.flush()
    }
}