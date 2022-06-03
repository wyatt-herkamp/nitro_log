use log::Record;
use serde_json::Value;
use std::io::{stdout, Stdout};

use crate::loggers::target::LoggerTargetBuilder;
use crate::loggers::{LoggerTarget, LoggerWriter};
use crate::{Error, Logger, PlaceHolders};

pub struct ConsoleLoggerBuilder;

impl LoggerTargetBuilder for ConsoleLoggerBuilder {
    #[inline]
    fn name(&self) -> &'static str {
        "console"
    }

    fn build(
        &mut self,
        _: &Logger,
        _value: Value,
        _placeholders: &PlaceHolders,
    ) -> Result<Box<dyn LoggerTarget>, Error> {
        let logger = ConsoleLogger { console: stdout() };
        Ok(Box::new(logger))
    }
}

pub struct ConsoleLogger {
    pub console: Stdout,
}

impl LoggerTarget for ConsoleLogger {
    fn start_write<'log>(&'log self, record: &'log Record) -> anyhow::Result<LoggerWriter<'log>> {
        let _x = Box::new(self.console.lock());
        Ok(LoggerWriter {
            internal: Box::new(self.console.lock()),
            record,
            logger: Box::new(self),
        })
    }
}
