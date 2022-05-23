use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::path::{MAIN_SEPARATOR, PathBuf};
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{Error, Placeholder};
use crate::placeholder::PlaceholderBuilder;

pub struct MessagePlaceholderBuilder;

impl PlaceholderBuilder for MessagePlaceholderBuilder {
    fn name<'a>(&self) -> &'a str {
        "message"
    }

    fn build(&self, _value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        Ok(Box::new(MessagePlaceholder {}))
    }
}

#[derive(Debug)]
pub struct MessagePlaceholder;


impl Placeholder for MessagePlaceholder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Borrowed(record.args().as_str().unwrap())
    }

    fn settings(&self) -> Option<Value> {
        None
    }
}

pub struct LevelPlaceHolderBuilder;

impl PlaceholderBuilder for LevelPlaceHolderBuilder {
    fn name<'a>(&self) -> &'a str {
        "level"
    }


    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        let config: LevelPlaceholderSettings = if let Some(config) = value {
            serde_json::from_value(config)?
        } else {
            LevelPlaceholderSettings::default()
        };
        #[cfg(feature = "colored")]
        {
            if config.colored {
                return Ok(Box::new(super::colored::ColorLevelPlaceholder{}));
            }
        }
        Ok(Box::new(LevelPlaceHolder))
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LevelPlaceholderSettings {
    #[cfg(feature = "colored")]
    pub colored: bool,
}


#[derive(Debug)]
pub struct LevelPlaceHolder;


impl Placeholder for LevelPlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Borrowed(record.level().as_str())
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}

pub struct ModulePlaceHolderBuilder;

impl PlaceholderBuilder for ModulePlaceHolderBuilder {
    fn name<'a>(&self) -> &'a str {
        "module"
    }


    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        if let Some(value) = value {
            if value.get("path").and_then(|value| value.as_bool()).unwrap_or(false) {
                return Ok(Box::new(PathModulePlaceHolder {}));
            }
        }
        Ok(Box::new(ModulePlaceHolder {}))
    }
}

#[derive(Debug)]
pub struct ModulePlaceHolder;


impl Placeholder for ModulePlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Borrowed(record.module_path().unwrap_or(""))
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}

#[derive(Debug)]
pub struct PathModulePlaceHolder;


impl Placeholder for PathModulePlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Owned(record.module_path().unwrap_or("").replace("::", &MAIN_SEPARATOR.to_string()))
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}
