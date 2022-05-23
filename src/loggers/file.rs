

use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use log::Record;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;
use crate::loggers::{LoggerTarget, LoggerTargetBuilder, LoggerWriter};
use crate::{PlaceHolders};
use crate::format::{Format, FormatSection};

pub struct FileLoggerBuilder;

impl LoggerTargetBuilder for FileLoggerBuilder {
    fn name(&self) -> String {
        "file_logger".to_string()
    }

    fn build(&self, value: Value, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error> {
        let file_config: FileConfig = serde_json::from_value(value)?;
        let logger = FileLogger {
            file_format: Format::new(placeholders, &file_config.file, true)?
        };
        Ok(Box::new(logger))
    }
}

pub struct FileLogger {
    pub file_format: Format,
}


impl LoggerTarget for FileLogger {
    fn start_write<'a>(&'a self, record: &Record) -> anyhow::Result<LoggerWriter<'a>> {
        let file = OpenOptions::new().create(true).append(true).open(generate_path(&self.file_format, record)?)?;
        Ok(LoggerWriter{
            writer: Box::new(file),
            logger: Box::new(self)
        })
    }

    fn return_write(& self, _write: &mut Box<dyn Write>) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileConfig {
    pub file: String,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            file: "log.log".to_string(),
        }
    }
}

pub fn generate_path(format: &Format, record: &Record) -> anyhow::Result<PathBuf> {
    let mut path = PathBuf::new();
    for values in &format.format {
        match values {
            FormatSection::Text(value) => {
                path = path.join(&value);
            }
            FormatSection::Variable(_variable) => {
                todo!("Variable Pathing is not available yet")
            }
            FormatSection::Placeholder(placeholder) => {
                path = path.join(placeholder.build_message(record));
            }
        }
    }
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    Ok(path)
}