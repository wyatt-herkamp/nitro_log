use std::collections::HashMap;
use serde_json::Value;
use crate::{Error, PlaceHolder};
use crate::placeholder::PlaceHolderBuilder;

pub struct MessagePlaceholderBuilder;

impl PlaceHolderBuilder for MessagePlaceholderBuilder {
    fn name(&self) -> String {
        "message".to_string()
    }

    fn build(&self, value: Option<HashMap<String, Value>>) -> Result<Box<dyn PlaceHolder>, Error> {
        todo!()
    }
}

pub struct LevelPlaceHolderBuilder;

impl PlaceHolderBuilder for LevelPlaceHolderBuilder {
    fn name(&self) -> String {
        "level".to_string()
    }

    fn build(&self, value: Option<HashMap<String, Value>>) -> Result<Box<dyn PlaceHolder>, Error> {
        todo!()
    }
}

pub struct ModulePlaceHolderBuilder;

impl PlaceHolderBuilder for ModulePlaceHolderBuilder {
    fn name(&self) -> String {
        "module".to_string()
    }

    fn build(&self, value: Option<HashMap<String, Value>>) -> Result<Box<dyn PlaceHolder>, Error> {
        todo!()
    }
}