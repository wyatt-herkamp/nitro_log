use std::collections::HashMap;
use std::env::var;
use std::io::{stdout, Stdout, Write};
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::loggers::{Logger, LoggerTarget, LoggerTargetBuilder, LoggerWriter};
use crate::{Error, NitroLogger, PlaceHolders};
use crate::format::{Format, FormatError, FormatSection};

pub struct ConsoleLoggerBuilder;

impl LoggerTargetBuilder for ConsoleLoggerBuilder {
    fn name(&self) -> String {
        "console".to_string()
    }

    fn build(&self, value: Value, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error> {
        let logger = ConsoleLogger {
            console: stdout(),
        };
        Ok(Box::new(logger))
    }
}

pub struct ConsoleLogger {
    pub console: Stdout,
}


impl LoggerTarget for ConsoleLogger {
    fn start_write<'a>(&'a self, record: &Record) -> anyhow::Result<LoggerWriter<'a>> {
        let x = Box::new(self.console.lock());
        Ok(LoggerWriter{
            writer: Box::new(self.console.lock()),
            logger: Box::new(self)
        })
    }

    fn return_write(& self, write: &mut Box<dyn Write>) -> anyhow::Result<()> {
        return Ok(());
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConsoleConfig {
    pub format: String,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        return ConsoleConfig {
            format: "{{module()}} {{level()}}: {{message()}}".to_string(),
        };
    }
}
