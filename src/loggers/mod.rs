use std::collections::HashMap;
use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;
use log::{Level, Record};
use serde_json::Value;
use crate::config::LoggerConfig;

use crate::error::Error;
use crate::format::{Format, FormatSection};
use crate::loggers::console::ConsoleLoggerBuilder;
use crate::PlaceHolders;

pub mod tree;
pub mod console;
pub mod file;

pub struct Logger {
    pub module: Option<String>,
    pub levels: Vec<Level>,
    pub targets: Vec<Box<dyn LoggerTarget>>,
    pub always_execute: bool,
}


impl Logger {
    pub fn module_matches(&self, module: &str) -> bool {
        if let Some(m) = self.module.as_ref() {
            if m.eq(module) {
                return true;
            }
        }

        return false;
    }
}

pub type LoggerTargetBuilders = Vec<Box<dyn LoggerTargetBuilder>>;

pub fn default_logger_targets() -> LoggerTargetBuilders {
    let mut logger_targets: LoggerTargetBuilders = Vec::new();
    logger_targets.push(Box::new(ConsoleLoggerBuilder {}));
    logger_targets.push(Box::new(file::FileLoggerBuilder {}));

    return logger_targets;
}

pub trait LoggerTargetBuilder {
    fn name(&self) -> String;
    fn build(&self, value: HashMap<String, Value>, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error>;
}

pub trait LoggerTarget: Sync + Send {
    fn log(
        &self,
        record: &Record,
    ) -> anyhow::Result<()>;
    fn name(&self) -> String;
}

pub fn write_log_standard<W: Write>(out: &mut W, format: &Format, record: &Record) -> anyhow::Result<()> {
    for values in &format.format {
        match values {
            FormatSection::Text(value) => {
                out.write_all(value.as_bytes())?;
            }
            FormatSection::Variable(variable) => {
                out.write_all(format!("Coming Soon {}", variable).as_bytes())?;
            }
            FormatSection::Placeholder(placeholder) => {
                out.write_all(placeholder.build_message(&record).as_bytes())?;
            }
        }
    }
    out.write_all("\n".as_bytes())?;
    out.flush()?;
    Ok(())
}

