use log::{as_error, as_serde, error, info, log_enabled, trace, warn};

use nitro_log::{LoggerBuilders, NitroLogger};

use std::path::PathBuf;
use log::Level::{Info, Trace};
use serde::{Deserialize, Serialize};
use nitro_log::error::Error;
use nitro_log::format::FormatError;

#[derive(Serialize)]
pub struct KvTest {
    pub hi: String,
}


#[test]
fn test() {
    let test = KvTest {
        hi: "My Value".to_string(),
    };
    NitroLogger::load_file(PathBuf::new().join("example.config.json"), LoggerBuilders::default()).unwrap();
    if log_enabled!(Trace) {
        let error1 = FormatError::MissingKey("Value".to_string());
        trace!(value = as_serde!(test), error=as_error!(error1); "Trace HEY");
    }
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");
}
