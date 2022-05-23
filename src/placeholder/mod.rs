pub mod standard_placeholders;
#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "colored")]
pub mod colored;

use std::borrow::Cow;

use std::fmt::{Debug};
use log::Record;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::Error;

pub type PlaceHolders = Vec<Box<dyn PlaceholderBuilder>>;
#[allow(unused_mut)]
pub fn default_placeholders() -> PlaceHolders {
    let mut placeholders: PlaceHolders = vec![Box::new(standard_placeholders::MessagePlaceholderBuilder {}),Box::new(standard_placeholders::LevelPlaceHolderBuilder {}),Box::new(standard_placeholders::ModulePlaceHolderBuilder {}),Box::new(standard_placeholders::EnvironmentPlaceholderBuilder {})];
    #[cfg(feature = "chrono")]
    placeholders.push(Box::new(chrono::ChronoPlaceHolderBuilder {}));
    placeholders
}

pub trait PlaceholderBuilder {
    /// The name of the placeholder
    fn name<'a>(&self) -> &'a str;
    /// Create a new Placeholder
    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error>;
}

pub trait Placeholder: Send + Sync + Debug {
    fn build_message<'a>(&'a self, record: &'a Record) -> Cow<'a, str>;

    /// Returns Settings received during creation
    fn settings(&self) -> Option<Value>;
}

pub fn parse_config<D: DeserializeOwned + Default>(value: Option<Value>) -> Result<D, Error> {
    if let Some(config) = value {
        serde_json::from_value(config).map_err(|error| Error::ConfigError("Placeholder".to_string(), error.to_string()))
    } else {
        Ok(D::default())
    }
}

pub fn parse_config_no_default<D: DeserializeOwned>(value: Option<Value>) -> Result<D, Error> {
    if let Some(config) = value {
        serde_json::from_value(config).map_err(|error| Error::ConfigError("Placeholder".to_string(), error.to_string()))
    } else {
        Err(Error::ConfigError("Placeholder".to_string(), "Missing Placeholder config".to_string()))
    }
}