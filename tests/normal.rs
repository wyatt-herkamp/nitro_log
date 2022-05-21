use log::{as_serde, error, info, log_enabled, trace, warn};

use nitro_log::NitroLogger;

use std::path::PathBuf;
use log::Level::{Info, Trace};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KvTest {
    pub hi: String,
}

fn main() {
    let test = KvTest {
        hi: "My Value".to_string()
    };
    NitroLogger::load_file(PathBuf::new().join("example.config.json"), None).unwrap();
    if log_enabled!(Trace) {
        trace!(value = as_serde!(test); "Trace HEY");
    }
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");
}
