use std::collections::HashMap;
use log::Record;
use crate::error::Error;
use crate::loggers::{Logger, LoggerTarget};

pub struct ConsoleLogger {
    pub format: String,
}

impl ConsoleLogger {
    pub fn init(settings: HashMap<String, String>) -> Result<ConsoleLogger, Error> {
        return Ok(ConsoleLogger { format: "".to_string() });
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        return ConsoleLogger::init(HashMap::new()).unwrap();
    }
}

impl LoggerTarget for ConsoleLogger {
    fn log(&self, logger: &Logger, record: &Record) -> Result<(), Error> {
        println!("{} - {}", record.level(), record.args());
        Ok(())
    }
}