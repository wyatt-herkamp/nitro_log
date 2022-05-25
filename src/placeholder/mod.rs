#[cfg(feature = "chrono")]
pub mod chrono;

pub mod standard_placeholders;
#[cfg(feature = "style-term")]
pub mod style_term;

use std::borrow::Cow;

use crate::Error;
use log::Record;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

pub type PlaceHolders = Vec<Box<dyn PlaceholderBuilder>>;

#[allow(unused_mut)]
pub fn default_placeholders() -> PlaceHolders {
    let mut placeholders: PlaceHolders = vec![
        Box::new(standard_placeholders::MessagePlaceholderBuilder {}),
        Box::new(standard_placeholders::LevelPlaceHolderBuilder {}),
        Box::new(standard_placeholders::ModulePlaceHolderBuilder {}),
        Box::new(standard_placeholders::EnvironmentPlaceholderBuilder {}),
    ];
    #[cfg(feature = "chrono")]
    placeholders.push(Box::new(chrono::ChronoPlaceHolderBuilder {}));
    placeholders
}

pub trait PlaceholderBuilder {
    /// The name of the placeholder
    fn name<'message>(&self) -> &'message str;
    /// Create a new Placeholder
    fn build(&self, value: Option<Value>) -> Result<Box<dyn Placeholder>, Error>;
}

pub trait Placeholder: Send + Sync + Debug {
    fn build_message<'message>(&'message self, record: &'message Record) -> Cow<'message, str>;

    /// Returns Settings received during creation
    fn settings(&self) -> Option<Value>;
}

pub fn parse_config<D: DeserializeOwned + Default>(value: Option<Value>) -> Result<D, Error> {
    if let Some(config) = value {
        serde_json::from_value(config)
            .map_err(|error| Error::ConfigError("Placeholder".to_string(), error.to_string()))
    } else {
        Ok(D::default())
    }
}

pub fn parse_config_no_default<D: DeserializeOwned>(value: Option<Value>) -> Result<D, Error> {
    if let Some(config) = value {
        serde_json::from_value(config)
            .map_err(|error| Error::ConfigError("Placeholder".to_string(), error.to_string()))
    } else {
        Err(Error::ConfigError(
            "Placeholder".to_string(),
            "Missing Placeholder config".to_string(),
        ))
    }
}
