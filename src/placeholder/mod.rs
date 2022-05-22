pub mod standard_placeholders;

use std::collections::HashMap;
use std::fmt::Debug;
use log::Record;
use serde_json::Value;
use crate::Error;

pub type PlaceHolders = Vec<Box<dyn PlaceHolderBuilder>>;

pub fn default_placeholders() -> PlaceHolders {
    let mut placeholders: PlaceHolders = Vec::new();
    placeholders.push(Box::new(standard_placeholders::MessagePlaceholderBuilder {}));
    placeholders.push(Box::new(standard_placeholders::LevelPlaceHolderBuilder {}));
    placeholders.push(Box::new(standard_placeholders::ModulePlaceHolderBuilder {}));

    return placeholders;
}

pub trait PlaceHolderBuilder {
    fn name(&self) -> String;
    fn build(&self, value: Option<HashMap<String, Value>>) -> Result<Box<dyn PlaceHolder>, Error>;
}

pub trait PlaceHolder: Send + Sync + Debug {
    fn build_message<'a>(&self, record: &'a Record) -> &'a str;
}