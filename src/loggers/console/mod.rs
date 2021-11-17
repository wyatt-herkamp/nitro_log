use std::collections::HashMap;
use log::Record;
use serde_json::Value;
use crate::error::Error;
use crate::loggers::{Logger, LoggerTarget};

pub struct ConsoleLogger {
    pub format: String,
}

impl ConsoleLogger {
    pub fn init(settings: HashMap<String, String>) -> Result<ConsoleLogger, Error> {
        let string = settings.get("format").unwrap_or(&"%level%: %message%".to_string()).to_string();
        let logger = ConsoleLogger { format: string };
        return Ok(logger);
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        return ConsoleLogger::init(HashMap::new()).unwrap();
    }
}

impl LoggerTarget for ConsoleLogger {
    fn log(&self, message: String) -> Result<(), Error> {
        println!("{}", message);
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

    fn format(&self) -> String {
        return self.format.clone();
    }
}