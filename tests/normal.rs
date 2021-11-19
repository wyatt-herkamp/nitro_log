use log::{error, info, log_enabled, trace, warn};

use nitro_log::NitroLogger;

use std::path::PathBuf;
use log::Level::{Info, Trace};

fn main() {
    NitroLogger::load_file(PathBuf::new().join("example.config.json"), None).unwrap();
    if log_enabled!(Trace) {
        trace!("Trace HEY");
    }
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");

}
