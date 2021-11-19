#[cfg(feature = "time")]
pub mod time;

use crate::Logger;
use log::Record;
use std::collections::HashMap;

pub type Placeholders = Vec<Box<dyn Placeholder>>;

pub trait FindPlaceholder {
    fn get_placeholder(&self, name:  &str) -> Option<&Box<dyn Placeholder>>;
}

impl FindPlaceholder for Placeholders {
    fn get_placeholder(&self, name: &str) -> Option<&Box<dyn Placeholder>> {
        for x in self {
            if name.eq(x.name()) {
                return Some(x);
            }
        }
        return None;
    }
}

/// PLaceHolders are variables that are added to a message or file path on log
/// They will appear between % % _{} within those will be properties
/// Example %datetime_{format='$Y'}%
pub trait Placeholder: Sync + Send {
    /// Called when replacing a PlaceHolder
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
    ) -> Option<String>;
    /// Called when replacing the placeholder for a file
    /// Will handle removing bad file characters
    fn replace_file(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
    ) -> Option<String>;
    /// Gets the Name of the PlaceHolder
    fn name(&self) -> &'static str;
}

/// The Module PlaceHolder just returns the Module Path from Record
pub struct ModulePlaceHolder;

impl Placeholder for ModulePlaceHolder {
    fn replace(
        &self,
        _properties: HashMap<String, String>,
        record: &Record,
        _logger: &Logger,
    ) -> Option<String> {
        Some(record.module_path().unwrap().to_string())
    }

    fn replace_file(&self, _properties: HashMap<String, String>, record: &Record, _logger: &Logger) -> Option<String> {
        Some(record.module_path().unwrap().to_string().replace("::", "/"))
    }

    fn name(&self) -> &'static str {
        return "module";
    }
}

/// Returns the Level of logging
/// You can add the color=true property to add color the level name
pub struct LevelPlaceholder;

impl Placeholder for LevelPlaceholder {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        _logger: &Logger,
    ) -> Option<String> {
        #[cfg(feature = "colors")]
            {
                use colored::Colorize;
                use log::Level;
                use std::str::FromStr;
                let default = "false".to_string();
                let x = properties.get("color").unwrap_or(&default);
                let value: bool = bool::from_str(x).unwrap();
                if value {
                    let string = record.metadata().level().to_string();
                    let value = match record.metadata().level() {
                        Level::Error => string.red().to_string(),
                        Level::Warn => string.yellow().to_string(),
                        Level::Info => string,
                        Level::Debug => string.green().to_string(),
                        Level::Trace => string.cyan().to_string(),
                    };
                    return Some(value);
                }
            }

        Some(record.metadata().level().to_string())
    }

    fn replace_file(&self, _properties: HashMap<String, String>, record: &Record, _logger: &Logger) -> Option<String> {
        Some(record.metadata().level().to_string())
    }

    fn name(&self) -> &'static str {
        return "level";
    }
}

/// Returns the Message provided by log::Record
pub struct MessagePlaceholder;

impl Placeholder for MessagePlaceholder {
    fn replace(
        &self,
        _properties: HashMap<String, String>,
        record: &Record,
        _logger: &Logger,
    ) -> Option<String> {
        Some(record.args().to_string())
    }

    fn replace_file(&self, _properties: HashMap<String, String>, _record: &Record, _logger: &Logger) -> Option<String> {
        return None;
    }

    fn name(&self) -> &'static str {
        return "message";
    }
}

// %env_{key=''}%
/// Returns environment keys.
/// key='' as a property to set the env name
pub struct EnvPlaceholder;

impl Placeholder for EnvPlaceholder {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        _record: &Record,
        _logger: &Logger,
    ) -> Option<String> {
        let option = properties.get("key");
        if option.is_none() {
            return None;
        }
        let key = option.unwrap();
        let result = std::env::var(key);
        if result.is_err() {
            return None;
        }
        return Some(result.unwrap());
    }

    fn replace_file(&self, properties: HashMap<String, String>, _record: &Record, _logger: &Logger) -> Option<String> {
        let option = properties.get("key");
        if option.is_none() {
            return None;
        }
        let key = option.unwrap();
        let result = std::env::var(key);
        if result.is_err() {
            return None;
        }
        return Some(sanitize_filename::sanitize(result.unwrap()));
    }

    fn name(&self) -> &'static str {
        return "env";
    }
}
