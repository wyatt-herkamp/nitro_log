use log::{LevelFilter, Metadata, Record};


use crate::config::Config;
use crate::error::Error;
use crate::loggers::tree::LoggerTree;
use crate::loggers::{Logger};
use crate::loggers::target::{default_logger_targets, LoggerTargetBuilders};

use crate::placeholder::{default_placeholders, Placeholder, PlaceHolders};


pub mod config;
pub mod error;
pub mod kv;
pub mod format;
pub mod loggers;
pub mod placeholder;

pub type ErrorHandler = Box<dyn Send + Sync + Fn(&anyhow::Error)>;

fn default_error_handler(error: &anyhow::Error) {
    eprintln!("Nitro Logger: {}", error);
}

pub struct LoggerBuilders {
    pub placeholders: PlaceHolders,
    pub targets: LoggerTargetBuilders,
}

impl Default for LoggerBuilders {
    fn default() -> Self {
        LoggerBuilders {
            placeholders: default_placeholders(),
            targets: default_logger_targets(),
        }
    }
}

pub struct NitroLogger {
    loggers: LoggerTree,
   pub(crate) error_handler: Box<dyn Send + Sync + Fn(&anyhow::Error)>,

}

impl NitroLogger {
    /// Load the Config via the Config the object
    pub fn load(config: Config, builders: LoggerBuilders) -> Result<(), Error> {
        let (root, loggers) = config::create_loggers(config, builders)?;
        let result = Self::new(LoggerTree::new(root, loggers), Box::new(default_error_handler));
        log::set_boxed_logger(Box::new(result))?;
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
    pub fn load_with_error_handler(config: Config, builders: LoggerBuilders, error_handler: ErrorHandler) -> Result<(), Error> {
        let (root, loggers) = config::create_loggers(config, builders)?;
        let result = Self::new(LoggerTree::new(root, loggers), error_handler);
        log::set_boxed_logger(Box::new(result))?;
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
    /// Create a new Nitro Logger with the already setup LoggerTree
    pub fn new(
        loggers: LoggerTree, error_handler: ErrorHandler) -> NitroLogger {
        NitroLogger {
            loggers,
            error_handler,
        }
    }
}


impl log::Log for NitroLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let option = self.loggers.find_logger(metadata.target());
        if option.is_none() {
            return false;
        }
        let loggers = option.unwrap();
        for logger in loggers {
            if logger.levels.contains(&metadata.level()) {
                return true;
            }
        }
        true
    }

    fn log(&self, record: &Record) {
        let option = self.loggers.find_logger(record.module_path().unwrap());
        if option.is_none() {
            panic!("No Loggers Found!");
        }

        let loggers = option.unwrap();
        for logger in loggers {
            if logger.levels.contains(&record.metadata().level()) {
                logger.log(record,self);
            }
        }
    }

    fn flush(&self) {}
}

