use std::fs::OpenOptions;
use log::{as_error, as_serde, debug, error, info, log_enabled, trace, warn};
use nitro_log::{LoggerBuilders, NitroLogger};

use log::Level::Trace;
use serde::Serialize;
use std::path::PathBuf;

use nitro_log::format::FormatError;

#[derive(Serialize)]
pub struct KvTest {
    pub hi: String,
}

#[test]
fn test() {
    let config = PathBuf::from("example.config.json");
    let file = OpenOptions::new().read(true).open(config).unwrap();
    let config = serde_json::from_reader(file).unwrap();
    NitroLogger::load(
        config,
        LoggerBuilders::default(),
    )
        .unwrap();
    let test = KvTest {
        hi: "My Value".to_string(),
    };

    if log_enabled!(Trace) {
        let error1 = FormatError::MissingKey("Value".to_string());
        trace!(value = as_serde!(test), error=as_error!(error1); "Trace HEY");
    }
    debug!("Debug Hey");
    info!("INFO HEY {}", test.hi);
    warn!("Warn HEY");
    error!("Error HEY");
}
