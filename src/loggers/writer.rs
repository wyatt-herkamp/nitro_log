use crate::loggers::LoggerTarget;
use log::Record;
use std::io::Write;

/// This is a wrapper around the actual Writer provided. Allowing for a consistent write experience through the log call
/// It will automatically return the writer upon drop. Running the post write tasks. Some implementations will not do anything.
pub struct LoggerWriter<'log> {
    pub internal: Box<dyn Write>,
    pub record: &'log Record<'log>,
    pub logger: Box<&'log dyn LoggerTarget>,
}

impl<'log> Write for LoggerWriter<'log> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.internal.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.internal.flush()
    }
}
