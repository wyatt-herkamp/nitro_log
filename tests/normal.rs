use log::Level::{Error, Info, Trace};
use log::{error, info, trace, warn, Level};
use nitro_log::loggers::console::ConsoleLogger;
use nitro_log::loggers::Logger;
use nitro_log::NitroLogger;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    NitroLogger::load_file(PathBuf::new().join("example.config.json"), None).unwrap();
    trace!("Trace HEY");
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");
}
