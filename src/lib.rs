use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use log::{LevelFilter, Metadata, Record};
use regex::Regex;

use crate::config::Config;
use crate::error::Error;
use crate::loggers::tree::LoggerTree;
use crate::loggers::Logger;

use lazy_static::lazy_static;
use log::kv::source::as_map;
use log::kv::{Key, Source, Value, Visitor};
use crate::placeholder::{PlaceHolder, PlaceHolders};

pub mod config;
pub mod error;
pub mod kv;
pub mod format;
pub mod loggers;
pub mod placeholder;

pub struct NitroLogger {
    pub loggers: LoggerTree,
}

impl NitroLogger {
    pub fn load_file(config: PathBuf, placeholders: Option<PlaceHolders>) -> Result<(), Error> {
        let config: Config = serde_json::from_reader(File::open(config)?)?;
        NitroLogger::load(config, placeholders)
    }
    pub fn load(config: Config, placeholders: Option<PlaceHolders>) -> Result<(), Error> {
        todo!()
        //  return NitroLogger::load_with_loggers(config.load(), placeholders);
    }
    pub fn load_with_loggers(
        loggers: LoggerTree,
        placeholders: Option<PlaceHolders>,
    ) -> Result<(), Error> {
        let vec = load_place_holders(placeholders);
        log::set_boxed_logger(Box::new(NitroLogger {
            loggers,
        }))
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .map_err(|e| Error::SetLoggerError(e))
    }
}

fn load_place_holders(placeholders: Option<PlaceHolders>) -> PlaceHolders {
    let mut placeholders = placeholders.unwrap_or(Vec::new());


    return placeholders;
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
                for x in &logger.targets {
                    if let Err(error) = x.log(record) {
                        println!("Error {}", error);
                    }
                }
            }
        }
    }

    fn flush(&self) {}
}

