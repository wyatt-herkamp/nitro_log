
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use log::Record;

use serde::{Deserialize, Serialize};


use crate::error::Error;
use crate::loggers::{Logger, LoggerTarget};
use crate::{NitroLogger, Placeholders};

pub struct FileLogger {
    pub config: FileConfig,
}

impl FileLogger {
    pub fn init(config: FileConfig) -> Result<FileLogger, Error> {
        let logger = FileLogger { config };
        return Ok(logger);
    }
}

impl Default for FileLogger {
    fn default() -> Self {
        return FileLogger::init(Default::default()).unwrap();
    }
}

impl LoggerTarget for FileLogger {
    fn log(
        &self,
        record: &Record,
        logger: &Logger,
        placeholder: &Placeholders,
    ) -> Result<(), Error> {
        let message = NitroLogger::parse_message(&self.config.file, logger, record, placeholder);
        let file_split: Vec<&str> = message.split("/").collect();
        let mut path = PathBuf::new();
        for i in 0..(file_split.len() - 1) {
            let x1 = file_split.get(i).unwrap();
            path = path.join(x1);
        }
        create_dir_all(&path)?;
        path = path.join(file_split.last().unwrap());
        let mut file = if !path.exists() {
            File::create(&path)?
        } else {
            OpenOptions::new().append(true).open(&path)?
        };
        let string = NitroLogger::parse_message(&self.config.format, logger, record, placeholder);
        file.write(format!("{}\n", string).as_bytes())?;
        file.flush()?;
        Ok(())
    }

    fn name(&self) -> String {
        return "file-logger".to_string();
    }

    fn format(&self) -> String {
        return self.config.format.clone();
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
