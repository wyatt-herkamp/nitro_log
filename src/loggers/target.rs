
use log::Record;
use serde_json::Value;
use crate::loggers::{console, file};
use crate::{Error, PlaceHolders};
use crate::loggers::writer::LoggerWriter;


pub type LoggerTargetBuilders = Vec<Box<dyn LoggerTargetBuilder>>;
#[allow(unused_mut)]
pub fn default_logger_targets() -> LoggerTargetBuilders {
    let mut logger_targets: LoggerTargetBuilders = vec![Box::new(console::ConsoleLoggerBuilder {}),Box::new(file::FileLoggerBuilder {}) ];
    logger_targets
}

pub trait LoggerTargetBuilder {
    ///The name of the target
    fn name(&self) -> String;
    /// Creates a new LoggerTarget
    /// # Errors
    /// Errors for config issues
    fn build(&self, config: Value, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error>;
}

pub trait LoggerTarget: Sync + Send {
    /// Returns a Write trait so the Logger can write to it
    fn start_write<'a>(
        &'a self, record: &'a Record,
    ) -> anyhow::Result<LoggerWriter<'a>>;

    /// Returns the writer
    fn return_write(& self, writer: LoggerWriter) -> anyhow::Result<()>;
}


