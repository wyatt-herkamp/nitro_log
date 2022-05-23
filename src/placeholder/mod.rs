pub mod standard_placeholders;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use log::Record;
use serde_json::Value;
use crate::Error;

pub type PlaceHolders = Vec<Box<dyn PlaceHolderBuilder>>;

pub fn default_placeholders() -> PlaceHolders {
    let mut placeholders: PlaceHolders = Vec::new();
    placeholders.push(Box::new(standard_placeholders::MessagePlaceholderBuilder {}));
    placeholders.push(Box::new(standard_placeholders::LevelPlaceHolderBuilder {}));
    placeholders.push(Box::new(standard_placeholders::ModulePlaceHolderBuilder {}));

    placeholders
}

pub trait PlaceHolderBuilder {
    fn name<'a>(&self) -> &'a str;
    fn build(&self, value: Option<Value>) -> Result<Box<dyn PlaceHolder>, Error>;
}

pub trait PlaceHolder: Send + Sync + Debug {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str>;

    /// Returns Settings received during creation
    fn settings(&self) -> Option<Value>;
}