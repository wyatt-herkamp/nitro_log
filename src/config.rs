use std::collections::HashMap;

use log::Level;
use log::Level::{Debug, Error, Info, Trace, Warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::loggers::console::{ConsoleConfig, ConsoleLogger};
use crate::loggers::file::{FileConfig, FileLogger};
use crate::loggers::LoggerTarget;
use crate::{Logger, LoggerTree};

#[derive(Serialize, Deserialize)]
pub struct TargetConfig {
    #[serde(rename = "type")]
    pub target_type: String,
    pub properties: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultLogger {
    #[serde(default = "default_levels")]
    pub levels: Vec<Level>,
    pub targets: Vec<TargetConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct LoggerConfig {
    pub module: String,
    #[serde(default = "default_levels")]
    pub levels: Vec<Level>,
    pub targets: Vec<TargetConfig>,
}

fn default_levels() -> Vec<Level> {
    vec![Trace, Info, Debug, Warn, Error]
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
        };
    }
}

impl From<LoggerConfig> for Logger {
    fn from(logger: LoggerConfig) -> Self {
        return Logger {
            module: logger.module,
            levels: logger.levels,
            targets: to_targets(logger.targets),
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
    pub loggers: Vec<LoggerConfig>,
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
