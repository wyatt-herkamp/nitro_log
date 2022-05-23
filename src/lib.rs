use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use log::{LevelFilter, Metadata, Record};
use regex::Regex;

use crate::config::Config;
use crate::error::Error;
use crate::loggers::tree::LoggerTree;
use crate::loggers::{default_logger_targets, Logger, LoggerTargetBuilders};

use lazy_static::lazy_static;
use log::kv::source::as_map;
use log::kv::{Key, Source, Value, Visitor};
use crate::placeholder::{default_placeholders, PlaceHolder, PlaceHolders};
use crate::placeholder::standard_placeholders::{MessagePlaceholderBuilder, LevelPlaceHolderBuilder, ModulePlaceHolderBuilder};

pub mod config;
pub mod error;
pub mod kv;
pub mod format;
pub mod loggers;
pub mod placeholder;

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
    pub loggers: LoggerTree,
}

impl NitroLogger {
    pub fn load_file(config: PathBuf, builders: LoggerBuilders) -> Result<(), Error> {
        let config: Config = serde_json::from_reader(File::open(config)?)?;
        NitroLogger::load(config, builders)
    }
    pub fn load(config: Config,builders: LoggerBuilders) -> Result<(), Error> {
        let (root, loggers) = config::create_loggers(config, builders)?;
        Self::new(LoggerTree::new(root, loggers))

    }
    pub fn new(
        loggers: LoggerTree) -> Result<(), Error> {
        log::set_boxed_logger(Box::new(NitroLogger {
            loggers,
        }))
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .map_err(|e| Error::SetLoggerError(e))
    }
}


impl log::Log for NitroLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let option = self.loggers.find_logger(&metadata.target());
        if option.is_none() {
            return false;
        }
        let loggers = option.unwrap();
        for logger in loggers {
            if logger.levels.contains(&metadata.level()) {
                return true;
            }
        }
        return true;
    }

    fn log(&self, record: &Record) {
        let option = self.loggers.find_logger(&record.module_path().unwrap());
        if option.is_none() {
            panic!("No Loggers Found!");
        }

        let loggers = option.unwrap();
        for logger in loggers {
            if logger.levels.contains(&record.metadata().level()) {
                logger.log(record);
            }
        }
    }

    fn flush(&self) {}
}

