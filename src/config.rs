use std::collections::HashMap;

use log::Level;
use log::Level::{Debug, Error, Info, Trace, Warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;


use crate::loggers::LoggerTarget;
use crate::{Logger, LoggerTree};

/// Target Config
#[derive(Serialize, Deserialize)]
pub struct TargetConfig {
    /// Target Name Ex: console or file
    #[serde(rename = "type")]
    pub target_type: String,
    /// Properties. Refer to Target config struct for more information
    pub properties: HashMap<String, Value>,
}

/// For Default Loggers
#[derive(Serialize, Deserialize)]
pub struct DefaultLogger {
    /// Levels To Log
    #[serde(default = "default_levels")]
    pub levels: Vec<Level>,
    /// Targets
    pub targets: Vec<TargetConfig>,
    /// Do you want this to always execute
    #[serde(default = "always_execute_default")]
    pub always_execute: bool,
}

/// For Loggers with modules
#[derive(Serialize, Deserialize)]
pub struct LoggerConfig {
    pub module: String,
    /// Levels
    #[serde(default = "default_levels")]
    pub levels: Vec<Level>,
    /// Targets
    pub targets: Vec<TargetConfig>,
    /// Do you want to always execute based on module parents
    /// If you have a module nitro::admin::system and nitro::admin
    /// if nitro::admin has this set to true
    /// And you grab the loggers for nitro::admin::system
    /// it will return true
    #[serde(default = "always_execute_default")]
    pub always_execute: bool,
}

fn default_levels() -> Vec<Level> {
    vec![Trace, Info, Debug, Warn, Error]
}

fn always_execute_default() -> bool {
    false
}






#[derive(Serialize, Deserialize)]
pub struct Config {
    /// All the loggers
    pub loggers: Vec<LoggerConfig>,
    ///Default Loggers
    pub default_loggers: Vec<DefaultLogger>,
}

