pub mod loggers;
pub mod error;

use std::error::Error;
use log::{LevelFilter, logger, Metadata, Record, SetLoggerError};
use crate::loggers::{FindLogger, Logger, LoggerTree};

pub struct NitroLogger {
    pub loggers: LoggerTree,
}

impl NitroLogger {
    pub fn load_with_loggers(loggers: LoggerTree) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(NitroLogger { loggers })).map(|()| log::set_max_level(LevelFilter::Trace))
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
        let loggers = option.unwrap().find_logger(&record);
        if let Some(logger) = loggers{
            logger.target.log(logger, record);
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
