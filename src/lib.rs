pub mod loggers;
pub mod error;
mod config;

use std::fs::{File, read_to_string};
use std::path::PathBuf;
use log::{LevelFilter, logger, Metadata, Record, SetLoggerError};
use crate::config::Config;
use crate::error::Error;
use crate::loggers::{ Logger};
use crate::loggers::tree::LoggerTree;

pub struct NitroLogger {
    pub loggers: LoggerTree,
}

impl NitroLogger {
    pub fn load_file(config: PathBuf) -> Result<(), Error> {
        let config: Config = serde_json::from_reader(File::open(config)?)?;
        NitroLogger::load(config)
    }
    pub fn load(config: Config) -> Result<(), Error> {
        return NitroLogger::load_with_loggers(config.load());
    }
    pub fn load_with_loggers(loggers: LoggerTree) -> Result<(), Error> {
        log::set_boxed_logger(Box::new(NitroLogger { loggers })).map(|()| log::set_max_level(LevelFilter::Trace)).map_err(|e| Error::SetLoggerError(e))
    }
}

impl log::Log for NitroLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        println!("Target: {}", metadata.target());
        return true;
    }

    fn log(&self, record: &Record) {
        let string = record.module_path().unwrap().to_string();
        let option = self.loggers.find_logger(&string);
        if option.is_none() {
            panic!("No Loggers Found!");
        }

        let loggers = option.unwrap();
        for logger in loggers {
            if logger.levels.contains(&record.metadata().level()){
                for x in &logger.targets {
                    x.log(logger, record);
                }
            }
        }
    }

    fn flush(&self) {
        println!("Flushing!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
