use std::fs::{File, OpenOptions};
use log::{as_error, as_serde, error, info, log_enabled, trace, warn};
use nitro_log::{LoggerBuilders, NitroLogger};

use colored::Colorize;
use log::Level::Trace;
use serde::Serialize;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use nitro_log::format::FormatError;

#[derive(Serialize)]
pub struct KvTest {
    pub hi: String,
}

#[test]
fn test() {
    let buf = PathBuf::new().join("example.config.json");
    let config = serde_json::from_reader(OpenOptions::new().read(true).open(buf).unwrap()).unwrap();
    NitroLogger::load(
        config,
        LoggerBuilders::default(),
    )
    .unwrap();
    let test = KvTest {
        hi: "My Value".parse().unwrap(),
    };

    if log_enabled!(Trace) {
        let error1 = FormatError::MissingKey("Value".to_string());
        trace!(value = as_serde!(test), error=as_error!(error1); "Trace HEY");
    }
    info!("INFO HEY {}", test.hi);
    warn!("Warn HEY");
    error!("Error HEY");
}
