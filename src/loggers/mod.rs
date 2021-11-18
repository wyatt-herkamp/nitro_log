use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

use log::Level::{Info, Warn};
use log::{logger, Level, Record};
use serde::de::Unexpected::Map;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::error::Error;
use crate::loggers::console::{ConsoleConfig, ConsoleLogger};
use crate::Placeholders;

pub mod console;
pub mod file;
pub mod tree;

pub struct Logger {
    pub module: String,
    pub levels: Vec<Level>,
    pub targets: Vec<Box<dyn LoggerTarget>>,
}

impl Default for Logger {
    fn default() -> Self {
        return Logger {
            module: "".to_string(),
            levels: vec![],
            targets: vec![Box::new(ConsoleLogger::default())],
        };
    }
}

impl Logger {
    pub fn module_matches(&self, module: &str) -> bool {
        if self.module.eq(module) {
            return true;
        }
        return false;
    }
}

pub trait LoggerTarget: Sync + Send {
    fn log(
        &self,
        record: &Record,
        logger: &Logger,
        placeholder: &Placeholders,
    ) -> Result<(), Error>;
    fn name(&self) -> String;
    fn format(&self) -> String;
}
