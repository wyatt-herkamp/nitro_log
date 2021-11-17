use std::collections::HashMap;
use std::path::PathBuf;
use log::{error, info, Level, trace, warn};
use log::Level::{Error, Info, Trace};
use nitro_log::loggers::console::ConsoleLogger;
use nitro_log::loggers::Logger;
use nitro_log::NitroLogger;

fn main() {
    NitroLogger::load_file(PathBuf::new().join("example.config.json"), None).unwrap();
    trace!("Trace HEY");
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");
}