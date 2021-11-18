use std::collections::HashMap;
use std::fs::{File};
use std::path::PathBuf;

use log::{LevelFilter, Metadata, Record};
use regex::Regex;

use crate::config::Config;
use crate::error::Error;
use crate::loggers::tree::LoggerTree;
use crate::loggers::{Logger};
use crate::placeholders::{
    EnvPlaceholder, FindPlaceholder, LevelPlaceholder, MessagePlaceholder, ModulePlaceHolder,
    Placeholders,
};

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
    pub fn parse_message(
        string: &String,
        logger: &Logger,
        record: &Record,
        placeholders: &Placeholders,
    ) -> String {
        let re = Regex::new("%(?P<value>.+?)%").unwrap();
        let properties = Regex::new("(?P<key>[a-zA-Z0-9]+)=\'(?P<value>.[^\']*)\',?").unwrap();
        let mut new_string = string.clone();
        for x in re.captures_iter(&string) {
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
            let mut props = HashMap::new();
            for x in properties.captures_iter(&values) {
                props.insert(
                    x.name("key").unwrap().as_str().to_string(),
                    x.name("value").unwrap().as_str().to_string(),
                );
            }
            let option = placeholders.get_placeholder(name.clone());
            if let Some(placeholder) = option {
                let replacement = placeholder.replace(props, record, logger);
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
            if logger.levels.contains(&record.metadata().level()) {
                for x in &logger.targets {
                    if let Err(error) = x.log(record, logger, &self.placeholders) {
                        println!("Error {}", error);
                    }
                }
            }
        }
    }

    fn flush(&self) {
        println!("Flushing!");
    }
}
