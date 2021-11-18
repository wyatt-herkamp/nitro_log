use std::collections::HashMap;

use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;
use crate::loggers::{Logger, LoggerTarget};
use crate::{NitroLogger, Placeholders};

pub struct ConsoleLogger {
    pub format: String,
}

impl ConsoleLogger {
    pub fn init(config: ConsoleConfig) -> Result<ConsoleLogger, Error> {
        let logger = ConsoleLogger {
            format: config.format,
        };
        return Ok(logger);
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        return ConsoleLogger::init(Default::default()).unwrap();
    }
}

impl LoggerTarget for ConsoleLogger {
    fn log(
        &self,
        record: &Record,
        logger: &Logger,
        placeholder: &Placeholders,
    ) -> Result<(), Error> {
        println!(
            "{}",
            NitroLogger::parse_message(&self.format, logger, record, placeholder)
        );
        Ok(())
    }

    fn name(&self) -> String {
        return "console".to_string();
    }

    fn format(&self) -> String {
        return self.format.clone();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConsoleConfig {
    pub format: String,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        return ConsoleConfig {
            format: "%module% %level%: %message%".to_string(),
        };
    }
}
