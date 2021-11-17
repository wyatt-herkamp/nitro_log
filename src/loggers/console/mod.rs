use std::collections::HashMap;
use log::Record;
use serde_json::Value;
use crate::error::Error;
use crate::loggers::{Logger, LoggerTarget};

pub struct ConsoleLogger {
    pub format: String,
}

impl ConsoleLogger {
    pub fn init(settings: HashMap<String, Value>) -> Result<ConsoleLogger, Error> {
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

    fn settings(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("format".to_string(), Value::String(self.format.clone()));
        return map;
    }

    fn name(&self) -> String {
        return "console".to_string();
    }
}