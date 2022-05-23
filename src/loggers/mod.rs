

use std::io::Write;


use log::{Level, Record};
use log::kv::{ToKey};
use serde_json::Value;


use crate::error::Error;
use crate::format::{Format, FormatSection};
use crate::kv::default_structure_dump::DefaultStructureDump;
use crate::loggers::console::ConsoleLoggerBuilder;
use crate::PlaceHolders;

pub mod tree;
pub mod console;
pub mod file;

pub struct LoggerWriter<'a> {
    pub writer: Box<dyn Write>,
    pub logger: Box<&'a dyn LoggerTarget>,
}

impl<'a> Drop for LoggerWriter<'a> {
    fn drop(&mut self) {
        self.logger.return_write(&mut self.writer).unwrap();
    }
}

pub struct Logger {
    pub module: Option<String>,
    pub levels: Vec<Level>,
    pub targets: Vec<Box<dyn LoggerTarget>>,
    pub always_execute: bool,
    pub structure_dump: bool,
    pub format: Format,
}


impl Logger {
    pub fn module_matches(&self, module: &str) -> bool {
        if let Some(m) = self.module.as_ref() {
            if m.eq(module) {
                return true;
            }
        }

        false
    }
    pub fn log(&self, record: &Record) {
        let mut writers = Vec::new();
        for target in &self.targets {
            if let Ok(value) = target.start_write(record) {
                writers.push(value);
            }
        }
        for values in &self.format.format {
            match values {
                FormatSection::Text(value) => {
                    self.write(&mut writers, value.as_bytes());
                }
                FormatSection::Variable(variable) => {
                    if let Some(value) = record.key_values().get(variable.to_key()) {
                        self.write(&mut writers, value.to_string().as_bytes());
                        self.write(&mut writers, variable.as_bytes());
                    }
                }
                FormatSection::Placeholder(placeholder) => {
                    self.write(&mut writers, placeholder.build_message(record).as_bytes());
                }
            }
        }
        let mut writers = if self.structure_dump {
            let mut dump = DefaultStructureDump {
                write: writers
            };
            record.key_values().visit(&mut dump).unwrap();
            dump.write
        }else{
            writers
        };

        self.write(&mut writers, "\n".as_bytes());

        for mut writer in writers.into_iter() {
            if let Err(_error) =  writer.writer.flush() {
                todo!("Errors not handled at flush")
            }
        }
    }
    fn write(&self, writers: &mut Vec<LoggerWriter>, content: &[u8]) {
        for writer in writers.iter_mut() {
            if let Err(_error) = writer.writer.write_all(content) {
                todo!("Errors not handled at writing")
            }
        }
    }
}

pub type LoggerTargetBuilders = Vec<Box<dyn LoggerTargetBuilder>>;

pub fn default_logger_targets() -> LoggerTargetBuilders {
    let mut logger_targets: LoggerTargetBuilders = Vec::new();
    logger_targets.push(Box::new(ConsoleLoggerBuilder {}));
    logger_targets.push(Box::new(file::FileLoggerBuilder {}));

    logger_targets
}

pub trait LoggerTargetBuilder {
    fn name(&self) -> String;
    fn build(&self, value: Value, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error>;
}

pub trait LoggerTarget: Sync + Send {
    /// Returns a Write trait so the Logger can write to it
    fn start_write<'a>(
        &'a self, record: &Record,
    ) -> anyhow::Result<LoggerWriter<'a>>;

    /// Returns the writer
    fn return_write(&self, write: &mut Box<dyn Write>) -> anyhow::Result<()>;
}


