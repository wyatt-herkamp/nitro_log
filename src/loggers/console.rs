use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{stdout, Stdout};

use crate::loggers::target::LoggerTargetBuilder;
use crate::loggers::{LoggerTarget, LoggerWriter};
use crate::{Error, PlaceHolders};

pub struct ConsoleLoggerBuilder;

impl LoggerTargetBuilder for ConsoleLoggerBuilder {
    #[inline]
    fn name(&self) -> String {
        "console".to_string()
    }

    fn build(
        &self,
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
    fn start_write<'a>(&'a self, record: &'a Record) -> anyhow::Result<LoggerWriter<'a>> {
        let _x = Box::new(self.console.lock());
        Ok(LoggerWriter {
            internal: Box::new(self.console.lock()),
            record,
            logger: Box::new(self),
        })
    }

    fn return_write(&self, _: LoggerWriter) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConsoleConfig {
    pub format: String,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        ConsoleConfig {
            format: "{{module()}} {{level()}}: {{message()}}".to_string(),
        }
    }
}
