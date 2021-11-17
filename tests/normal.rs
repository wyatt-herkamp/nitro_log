use std::collections::HashMap;
use log::{error, info, Level, trace, warn};
use log::Level::{Error, Info, Trace};
use nitro_log::loggers::console::ConsoleLogger;
use nitro_log::loggers::Logger;
use nitro_log::NitroLogger;

fn main() {
    let result = ConsoleLogger::init(HashMap::new()).unwrap();
    let logger = Logger {
        module: "normal".to_string(),
        levels: vec![Level::Debug, Level::Warn, Trace, Info, Error],
        target: Box::new(result),
    };
    NitroLogger::load().unwrap();
    trace!("Trace HEY");
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");
}