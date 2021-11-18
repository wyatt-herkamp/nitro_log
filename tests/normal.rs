use log::{error, info, trace, warn};

use nitro_log::NitroLogger;

use std::path::PathBuf;

fn main() {
    NitroLogger::load_file(PathBuf::new().join("example.config.json"), None).unwrap();
    trace!("Trace HEY");
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");
}
