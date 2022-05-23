use std::fs::{create_dir_all, OpenOptions};

use std::path::PathBuf;

use log::Record;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;
use crate::loggers::{LoggerTarget, LoggerWriter};
use crate::{PlaceHolders};
use crate::config::FormatConfig;
use crate::format::{Format, FormatSection};
use crate::loggers::target::LoggerTargetBuilder;

pub struct FileLoggerBuilder;

impl LoggerTargetBuilder for FileLoggerBuilder {
    fn name(&self) -> String {
        "file_logger".to_string()
    }

    fn build(&self, value: Value, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error> {
        let file_config: FileConfig = serde_json::from_value(value)?;
        let logger = FileLogger {
            file_format: Format::new(placeholders, file_config.file, true)?
        };
        Ok(Box::new(logger))
    }
}

pub struct FileLogger {
    pub file_format: Format,
}


impl LoggerTarget for FileLogger {
    fn start_write<'a>(&'a self, record: &'a Record) -> anyhow::Result<LoggerWriter<'a>> {
        let file = OpenOptions::new().create(true).append(true).open(generate_path(&self.file_format, record)?)?;
        Ok(LoggerWriter {
            internal: Box::new(file),
            logger: Box::new(self),
            record,
        })
    }

    fn return_write(&self, _: LoggerWriter) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileConfig {
    #[serde(deserialize_with = "crate::config::format_config_string_or_struct")]
    pub file: FormatConfig,
}


pub fn generate_path(format: &Format, record: &Record) -> anyhow::Result<PathBuf> {
    let mut path = String::new();
    for values in format.format.iter() {
        match values {
            FormatSection::Text(value) => {
                path.push_str(value);
            }
            FormatSection::Variable(_variable) => {
                todo!("Variable Pathing is not available yet")
            }
            FormatSection::Placeholder(placeholder) => {
                path.push_str(placeholder.build_message(record).as_ref());
            }
        }
    }

    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    Ok(path)
}