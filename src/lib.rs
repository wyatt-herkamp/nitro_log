pub mod loggers;
pub mod error;
pub mod config;
pub mod placeholders;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::path::PathBuf;
use log::{LevelFilter, logger, Metadata, Record, SetLoggerError};
use regex::Regex;
use crate::config::Config;
use crate::error::Error;
use crate::loggers::{Logger, LoggerTarget};
use crate::loggers::tree::LoggerTree;
use crate::placeholders::{FindPlaceholder, LevelPlaceholder, MessagePlaceholder, ModulePlaceHolder, Placeholder, Placeholders};

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
    pub fn load_with_loggers(loggers: LoggerTree, placeholders: Option<Placeholders>) -> Result<(), Error> {
        log::set_boxed_logger(Box::new(NitroLogger { loggers, placeholders: load_place_holders(placeholders) })).map(|()| log::set_max_level(LevelFilter::Trace)).map_err(|e| Error::SetLoggerError(e))
    }
}

fn load_place_holders(placeholders: Option<Placeholders>) -> Placeholders {
    let mut placeholders = placeholders.unwrap_or(Vec::new());
    placeholders.push(Box::new(ModulePlaceHolder));
    placeholders.push(Box::new(MessagePlaceholder));
    placeholders.push(Box::new(LevelPlaceholder));
    return placeholders;
}

/// %module% %dateTime_{format=''}% %level%: %message%

impl NitroLogger {
    pub fn parse_message(&self, logger: &Logger, target: &Box<dyn LoggerTarget>, record: &Record) -> String {
        let re = Regex::new("%(?P<value>.+?)%").unwrap();
        let properties = Regex::new("(?P<key>[a-zA-Z0-9]+)=\'(?P<value>.[^\']*)\',?").unwrap();
        let string = target.format().clone();
        let mut new_string = target.format().clone();
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
                props.insert(x.name("key").unwrap().as_str().to_string(), x.name("value").unwrap().as_str().to_string());
            }
            let option = self.placeholders.get_placeholder(name.clone());
            if let Some(placeholder) = option {
                let replacement = placeholder.replace(props, record, logger);
                new_string = new_string.replace(og_text.as_str(), replacement.as_str());
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
                    x.log(self.parse_message(logger, x, record));
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
