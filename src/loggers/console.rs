

use std::io::{stdout, Stdout, Write};
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::loggers::{LoggerTarget, LoggerTargetBuilder, LoggerWriter};
use crate::{Error, PlaceHolders};


pub struct ConsoleLoggerBuilder;

impl LoggerTargetBuilder for ConsoleLoggerBuilder {
    fn name(&self) -> String {
        "console".to_string()
    }

    fn build(&self, _value: Value, _placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error> {
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
    fn start_write<'a>(&'a self, _record: &Record) -> anyhow::Result<LoggerWriter<'a>> {
        let _x = Box::new(self.console.lock());
        Ok(LoggerWriter{
            writer: Box::new(self.console.lock()),
            logger: Box::new(self)
        })
    }

    fn return_write(& self, _write: &mut Box<dyn Write>) -> anyhow::Result<()> {
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
