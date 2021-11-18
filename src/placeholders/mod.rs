#[cfg(feature = "time")]
pub mod time;

use crate::Logger;
use log::{Level, Record};
use std::collections::HashMap;
use std::str::FromStr;

pub type Placeholders = Vec<Box<dyn Placeholder>>;

pub trait FindPlaceholder {
    fn get_placeholder(&self, name: String) -> Option<&Box<dyn Placeholder>>;
}

impl FindPlaceholder for Placeholders {
    fn get_placeholder(&self, name: String) -> Option<&Box<dyn Placeholder>> {
        for x in self {
            if name.eq(x.name()) {
                return Some(x);
            }
        }
        return None;
    }
}

// %.+?%
/// %module% %date_time{format=''}% %level%: %message%
pub trait Placeholder: Sync + Send {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
    ) -> Option<String>;
    fn name(&self) -> &'static str;
}

pub struct ModulePlaceHolder;

impl Placeholder for ModulePlaceHolder {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
    ) -> Option<String> {
        Some(record.module_path().unwrap().to_string())
    }

    fn name(&self) -> &'static str {
        return "module";
    }
}

pub struct LevelPlaceholder;

impl Placeholder for LevelPlaceholder {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
    ) -> Option<String> {
        #[cfg(feature = "colors")]
        {
            use colored::Colorize;
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

    fn name(&self) -> &'static str {
        return "level";
    }
}

pub struct MessagePlaceholder;

impl Placeholder for MessagePlaceholder {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
    ) -> Option<String> {
        Some(record.args().to_string())
    }

    fn name(&self) -> &'static str {
        return "message";
    }
}

// %env_{key=''}%
pub struct EnvPlaceholder;

impl Placeholder for EnvPlaceholder {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
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

    fn name(&self) -> &'static str {
        return "env";
    }
}
