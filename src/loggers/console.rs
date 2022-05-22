use std::collections::HashMap;
use std::env::var;
use std::io::{stdout, Stdout, Write};
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::loggers::{Logger, LoggerTarget, LoggerTargetBuilder, write_log_standard};
use crate::{Error, NitroLogger, PlaceHolders};
use crate::format::{Format, FormatError, FormatSection};

pub struct ConsoleLoggerBuilder;

impl LoggerTargetBuilder for ConsoleLoggerBuilder {
    fn name(&self) -> String {
        "console".to_string()
    }

    fn build(&self, value: HashMap<String, Value>, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error> {
        let logger = ConsoleLogger {
            format: Format::new(placeholders, &value.get("format").unwrap().to_string(), false)?,
            console: stdout(),
        };
        Ok(Box::new(logger))
    }
}

pub struct ConsoleLogger {
    pub format: Format,
    pub console: Stdout,
}



impl LoggerTarget for ConsoleLogger {
    fn log(
        &self,
        record: &Record,
    ) -> anyhow::Result<()> {
        let mut out = self.console.lock();
        write_log_standard(&mut out, &self.format, record)
    }

    fn name(&self) -> String {
        return "console".to_string();
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
