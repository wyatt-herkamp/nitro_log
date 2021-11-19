use std::collections::HashMap;

use log::Level;
use log::Level::{Debug, Error, Info, Trace, Warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::loggers::console::{ConsoleConfig, ConsoleLogger};
use crate::loggers::file::{FileConfig, FileLogger};
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

impl From<TargetConfig> for Box<dyn LoggerTarget> {
    fn from(target: TargetConfig) -> Self {
        if target.target_type.eq_ignore_ascii_case("console") {
            let map = target.properties;
            let result = serde_json::to_value(map).unwrap();
            let config: ConsoleConfig = serde_json::from_value(result).unwrap();
            return Box::new(ConsoleLogger::init(config).unwrap());
        } else if target.target_type.eq_ignore_ascii_case("file-logger") {
            let map = target.properties;
            let result = serde_json::to_value(map).unwrap();
            let config: FileConfig = serde_json::from_value(result).unwrap();
            return Box::new(FileLogger::init(config).unwrap());
        } else {
            panic!("Unable to find target {}", target.target_type);
        }
    }
}

pub fn to_targets(configs: Vec<TargetConfig>) -> Vec<Box<dyn LoggerTarget>> {
    let mut targets = Vec::new();
    for x in configs {
        targets.push(x.into())
    }
    return targets;
}

impl From<DefaultLogger> for Logger {
    fn from(logger: DefaultLogger) -> Self {
        return Logger {
            module: "".to_string(),
            levels: logger.levels,
            targets: to_targets(logger.targets),
            always_execute: logger.always_execute,
        };
    }
}

impl From<LoggerConfig> for Logger {
    fn from(logger: LoggerConfig) -> Self {
        return Logger {
            module: logger.module,
            levels: logger.levels,
            targets: to_targets(logger.targets),
            always_execute: logger.always_execute,
        };
    }
}

pub fn to_default_loggers(loggers: Vec<DefaultLogger>) -> Vec<Logger> {
    let mut new_loggers = Vec::new();
    for x in loggers {
        new_loggers.push(x.into())
    }
    return new_loggers;
}

pub fn to_loggers(loggers: Vec<LoggerConfig>) -> Vec<Logger> {
    let mut new_loggers = Vec::new();
    for x in loggers {
        new_loggers.push(x.into())
    }
    return new_loggers;
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// All the loggers
    pub loggers: Vec<LoggerConfig>,
    ///Default Loggers
    pub default_loggers: Vec<DefaultLogger>,
}

impl Config {
    pub fn load(self) -> LoggerTree {
        LoggerTree::new(
            to_default_loggers(self.default_loggers),
            to_loggers(self.loggers),
        )
    }
}
