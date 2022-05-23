use std::borrow::Cow;
use log::Record;
use serde_json::Value;
use crate::Placeholder;
use colored::Colorize;
use log::Level;
use std::str::FromStr;

#[derive(Debug)]
pub struct ColorLevelPlaceholder;


impl Placeholder for ColorLevelPlaceholder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        let string = record.metadata().level().to_string();
        let value = match record.metadata().level() {
            Level::Error => string.red().to_string(),
            Level::Warn => string.yellow().to_string(),
            Level::Info => string,
            Level::Debug => string.green().to_string(),
            Level::Trace => string.cyan().to_string(),
        };
        Cow::Owned(value)
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}
