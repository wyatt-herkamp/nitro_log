use crate::placeholder::PlaceholderBuilder;
use crate::{Error, Placeholder};
use chrono::Local;
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::{Debug};

pub struct ChronoPlaceHolderBuilder;

impl PlaceholderBuilder for ChronoPlaceHolderBuilder {
    fn name<'message>(&self) -> &'message str {
        "chrono"
    }

    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        let config = if let Some(config) = value {
            serde_json::from_value(config)?
        } else {
            ChronoConfig::default()
        };
        Ok(Box::new(ChronoPlaceholder { config }))
    }
}

#[derive(Debug)]
pub struct ChronoPlaceholder {
    pub config: ChronoConfig,
}

impl Placeholder for ChronoPlaceholder {
    fn build_message<'message>(&'message self, _: &'message Record) -> Cow<'message, str> {
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
            format: "%Y-%m-%d %H:%M:%S".to_string(),
        }
    }
}
