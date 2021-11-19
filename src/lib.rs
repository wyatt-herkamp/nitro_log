use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use log::{LevelFilter, Metadata, Record};
use regex::Regex;

use crate::config::Config;
use crate::error::Error;
use crate::loggers::tree::LoggerTree;
use crate::loggers::Logger;
use crate::placeholders::{
    EnvPlaceholder, FindPlaceholder, LevelPlaceholder, MessagePlaceholder, ModulePlaceHolder,
    Placeholders,
};
use lazy_static::lazy_static;

pub mod config;
pub mod error;
pub mod loggers;
pub mod placeholders;

pub struct NitroLogger {
    pub loggers: LoggerTree,
    pub placeholders: Placeholders,
}

impl NitroLogger {
    pub fn load_file(config: PathBuf, placeholders: Option<Placeholders>) -> Result<(), Error> {
        let config: Config = serde_json::from_reader(File::open(config)?)?;
        NitroLogger::load(config, placeholders)
    }
    pub fn load(config: Config, placeholders: Option<Placeholders>) -> Result<(), Error> {
        return NitroLogger::load_with_loggers(config.load(), placeholders);
    }
    pub fn load_with_loggers(
        loggers: LoggerTree,
        placeholders: Option<Placeholders>,
    ) -> Result<(), Error> {
        log::set_boxed_logger(Box::new(NitroLogger {
            loggers,
            placeholders: load_place_holders(placeholders),
        }))
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .map_err(|e| Error::SetLoggerError(e))
    }
}

fn load_place_holders(placeholders: Option<Placeholders>) -> Placeholders {
    let mut placeholders = placeholders.unwrap_or(Vec::new());
    placeholders.push(Box::new(ModulePlaceHolder));
    placeholders.push(Box::new(MessagePlaceholder));
    placeholders.push(Box::new(LevelPlaceholder));
    placeholders.push(Box::new(EnvPlaceholder));
    #[cfg(feature = "time")]
        placeholders.push(Box::new(crate::placeholders::time::DateTimePlaceholder));

    return placeholders;
}

/// %module% %dateTime_{format=''}% %level%: %message%

impl NitroLogger {
    /// This is the code that parses and calls placeholders.
    /// I know it is ugly. However, I am not skilled with String parsing.
    pub fn parse_message(
        string: &String,
        logger: &Logger,
        record: &Record,
        placeholders: &Placeholders,
        file: bool,
    ) -> String {
        lazy_static! {
            static ref PLACEHOLDER : Regex= Regex::new("%(?P<value>.+?)%").unwrap();
            static ref  PROPERTIES_REGEX : Regex = Regex::new("(?P<key>[a-zA-Z0-9]+)=\'(?P<value>.[^\']*)\',?").unwrap();
        }
        let mut new_string = string.clone();
        for x in PLACEHOLDER.captures_iter(&string) {
            let og_text = x.get(0).unwrap().as_str().to_string();
            let x1 = x.name("value").unwrap().as_str();
            let split: Vec<&str> = x1.split("_{").collect();
            let (name, values) = if split.len() == 2 {
                let mut value = split.get(1).unwrap().to_string();
                value.pop();
                (split.get(0).unwrap().to_string(), value)
            } else {
                (x1.to_string(), "".to_string())
            };
            let mut props = HashMap::<String, String>::new();
            for x in PROPERTIES_REGEX.captures_iter(&values) {
                props.insert(
                    x.name("key").unwrap().as_str().to_string(),
                    x.name("value").unwrap().as_str().to_string(),
                );
            }
            let option = placeholders.get_placeholder(&name);
            if let Some(placeholder) = option {
                let replacement = if file {
                    placeholder.replace_file(props, record, logger)
                } else {
                    placeholder.replace(props, record, logger)
                };
                if let Some(value) = replacement {
                    new_string = new_string.replace(og_text.as_str(), value.as_str());
                }
            }
        }
        return new_string;
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
                for x in &logger.targets {
                    if let Err(error) = x.log(record, logger, &self.placeholders) {
                        println!("Error {}", error);
                    }
                }
            }
        }
    }

    fn flush(&self) {}
}
