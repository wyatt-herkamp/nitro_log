use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use chrono::Local;
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{Error, Placeholder};
use crate::placeholder::PlaceholderBuilder;

pub struct ChronoPlaceHolderBuilder;

impl PlaceholderBuilder for ChronoPlaceholder {
    fn name<'a>(&self) -> &'a str {
        "chrono"
    }

    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        let config = if let Some(config) = value {
            serde_json::from_value(config)?
        } else {
            ChronoConfig::default()
        };
    }
}

#[derive(Debug)]
pub struct ChronoPlaceholder {
    pub config: ChronoConfig,
}


impl Placeholder for ChronoPlaceholder {
    fn build_message<'a>(&self, _: &'a Record) -> Cow<'a, str> {
        Cow::Owned(Local::now().format(&self.config.format).to_string())
    }

    fn settings(&self) -> Option<Value> {
        serde_json::to_value(self.config.clone()).ok()
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChronoConfig {
    pub format: String,
}

impl Default for ChronoConfig {
    fn default() -> Self {
        ChronoConfig {
            format: "%Y-%m-%d %H:%M:%S".to_string()
        }
    }
}