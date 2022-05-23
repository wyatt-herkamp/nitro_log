use crate::Placeholder;
use colored::Colorize;
use log::Level;
use log::Record;
use serde_json::Value;
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Debug)]
pub struct ColorLevelPlaceholder;

impl Placeholder for ColorLevelPlaceholder {
    fn build_message<'message>(&'message self, record: &'message Record) -> Cow<'message, str> {
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
