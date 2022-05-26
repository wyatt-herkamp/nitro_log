use crate::loggers::writer::LoggerWriter;
use crate::loggers::{console, file};
use crate::{Error, PlaceHolders};
use log::Record;
use serde_json::Value;

pub type LoggerTargetBuilders = Vec<Box<dyn LoggerTargetBuilder>>;

#[allow(unused_mut)]
pub fn default_logger_targets() -> LoggerTargetBuilders {
    let mut logger_targets: LoggerTargetBuilders = vec![
        Box::new(console::ConsoleLoggerBuilder {}),
        Box::new(file::FileLoggerBuilder {}),
    ];
    logger_targets
}

pub trait LoggerTargetBuilder {
    ///The name of the target
    fn name(&self) -> &'static str;
    /// Creates a new LoggerTarget
    /// # Errors
    /// Errors for config issues
    fn build(
        &self,
        config: Value,
        placeholders: &PlaceHolders,
    ) -> Result<Box<dyn LoggerTarget>, Error>;
}

pub trait LoggerTarget: Sync + Send {
    /// Returns a Write trait so the Logger can write to it
    fn start_write<'log>(&'log self, record: &'log Record) -> anyhow::Result<LoggerWriter<'log>>;

    /// Returns the writer
    /// By default this function does nothing.
    fn return_write(&self, _: LoggerWriter) -> anyhow::Result<()> {
        Ok(())
    }
}
