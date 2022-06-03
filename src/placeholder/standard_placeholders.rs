use std::borrow::Cow;

use crate::placeholder::PlaceholderBuilder;
use crate::{Error, Placeholder};
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::path::MAIN_SEPARATOR;

pub struct MessagePlaceholderBuilder;

impl PlaceholderBuilder for MessagePlaceholderBuilder {
    fn name<'message>(&self) -> &'message str {
        "message"
    }

    fn build(&self, _value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        Ok(Box::new(MessagePlaceholder {}))
    }
}

#[derive(Debug)]
pub struct MessagePlaceholder;

impl Placeholder for MessagePlaceholder {
    fn build_message<'message>(&'message self, record: &'message Record) -> Cow<'message, str> {
        Cow::Owned(record.args().to_string())
    }

    fn settings(&self) -> Option<Value> {
        None
    }
}

pub struct LevelPlaceHolderBuilder;

impl PlaceholderBuilder for LevelPlaceHolderBuilder {
    fn name<'message>(&self) -> &'message str {
        "level"
    }

    fn build(&self, _value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        #[cfg(feature = "style-term")]
        {
            let value: LevelPlaceholderSettings = super::parse_config_no_default(_value)?;

            if !value.path && value.styles.is_some() {
                Ok(Box::new(
                    super::style_term::level::StyledLevelPlaceholder::from(value.styles.unwrap()),
                ))
            } else {
                Ok(Box::new(LevelPlaceHolder))
            }
        }
        #[cfg(not(feature = "style-term"))]
        Ok(Box::new(LevelPlaceHolder))
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LevelPlaceholderSettings {
    #[cfg(feature = "style-term")]
    #[cfg_attr(feature = "style-term", serde(default))]
    pub styles: Option<super::style_term::level::LevelColorConfig>,
    #[serde(default)]
    pub path: bool,
}

#[derive(Debug)]
pub struct LevelPlaceHolder;

impl Placeholder for LevelPlaceHolder {
    fn build_message<'message>(&'message self, record: &'message Record) -> Cow<'message, str> {
        Cow::Borrowed(record.level().as_str())
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}

pub struct ModulePlaceHolderBuilder;

impl PlaceholderBuilder for ModulePlaceHolderBuilder {
    fn name<'message>(&self) -> &'message str {
        "module"
    }

    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        if let Some(value) = value {
            if value
                .get("path")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
            {
                return Ok(Box::new(PathModulePlaceHolder {}));
            }
        }
        Ok(Box::new(ModulePlaceHolder {}))
    }
}

#[derive(Debug)]
pub struct ModulePlaceHolder;

impl Placeholder for ModulePlaceHolder {
    fn build_message<'message>(&'message self, record: &'message Record) -> Cow<'message, str> {
        Cow::Borrowed(record.module_path().unwrap_or(""))
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}

#[derive(Debug)]
pub struct PathModulePlaceHolder;

impl Placeholder for PathModulePlaceHolder {
    fn build_message<'message>(&'message self, record: &'message Record) -> Cow<'message, str> {
        Cow::Owned(
            record
                .module_path()
                .unwrap_or("")
                .replace("::", &MAIN_SEPARATOR.to_string()),
        )
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}

#[derive(Deserialize, Serialize)]
pub struct EnvironmentPlaceholderSettings {
    pub key: String,
    /// If save is enabled it will keep this value cached
    #[serde(default)]
    pub save: bool,
}

pub struct EnvironmentPlaceholderBuilder;

impl PlaceholderBuilder for EnvironmentPlaceholderBuilder {
    fn name<'message>(&self) -> &'message str {
        "env"
    }

    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        let config: EnvironmentPlaceholderSettings = super::parse_config_no_default(value)?;
        if config.save {
            let result = std::env::var(config.key).map_err(|error| {
                Error::ConfigError("Placeholder".to_string(), error.to_string())
            })?;
            Ok(Box::new(SavedEnvVariable(result)))
        } else {
            Ok(Box::new(NotSavedEnvVariable(config.key)))
        }
    }
}

#[derive(Debug)]
pub struct SavedEnvVariable(String);

impl Placeholder for SavedEnvVariable {
    fn build_message<'message>(&'message self, _: &'message Record) -> Cow<'message, str> {
        Cow::Borrowed(self.0.as_str())
    }

    fn settings(&self) -> Option<Value> {
        serde_json::to_value(EnvironmentPlaceholderSettings {
            key: self.0.clone(),
            save: true,
        })
        .ok()
    }
}

#[derive(Debug)]
pub struct NotSavedEnvVariable(String);

impl Placeholder for NotSavedEnvVariable {
    fn build_message<'message>(&'message self, _: &'message Record) -> Cow<'message, str> {
        Cow::Owned(std::env::var(&self.0).unwrap_or_else(|_| "undefined".to_string()))
    }

    fn settings(&self) -> Option<Value> {
        serde_json::to_value(EnvironmentPlaceholderSettings {
            key: self.0.clone(),
            save: false,
        })
        .ok()
    }
}
