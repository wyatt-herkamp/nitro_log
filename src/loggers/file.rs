use std::collections::HashMap;
use std::fmt::write;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use log::Record;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;
use crate::loggers::{Logger, LoggerTarget, LoggerTargetBuilder, write_log_standard};
use crate::{NitroLogger, PlaceHolders};
use crate::format::{Format, FormatSection};

pub struct FileLoggerBuilder;

impl LoggerTargetBuilder for FileLoggerBuilder {
    fn name(&self) -> String {
        "file_logger".to_string()
    }

    fn build(&self, value: HashMap<String, Value>, placeholders: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error> {
        let logger = FileLogger {
            content_format: Format::new(placeholders, &value.get("format").unwrap().to_string(), false)?,
            file_format: Format::new(placeholders, &value.get("file").unwrap().to_string(), true)?
        };
        Ok(Box::new(logger))
    }
}

pub struct FileLogger {
    pub content_format: Format,
    pub file_format: Format,
}




impl LoggerTarget for FileLogger {
    fn log(
        &self,
        record: &Record,
    ) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(generate_path(&self.file_format, record)?)?;
        write_log_standard(&mut file, &self.content_format, record)?;
        Ok(())
    }

    fn name(&self) -> String {
        return "file-logger".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileConfig {
    pub format: String,
    pub file: String,
}

impl Default for FileConfig {
    fn default() -> Self {
        return FileConfig {
            format: "%module% %level%: %message%".to_string(),
            file: "log.log".to_string(),
        };
    }
}

pub fn generate_path(format: &Format, record: &Record) -> anyhow::Result<PathBuf> {
    let mut path = PathBuf::new();
    for values in &format.format {
        match values {
            FormatSection::Text(value) => {
                path = path.join(&value);
            }
            FormatSection::Variable(variable) => {
                todo!("Variable Pathing is not available yet")
            }
            FormatSection::Placeholder(placeholder) => {
                path = path.join(placeholder.build_message(&record));
            }
        }
    }
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    Ok(path)
}