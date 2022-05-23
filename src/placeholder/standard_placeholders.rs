use std::collections::HashMap;
use std::fmt::{Debug};
use log::Record;
use serde_json::Value;
use crate::{Error, PlaceHolder};
use crate::placeholder::PlaceHolderBuilder;

pub struct MessagePlaceholderBuilder;

impl PlaceHolderBuilder for MessagePlaceholderBuilder {
    fn name(&self) -> String {
        "message".to_string()
    }

    fn build(&self, _value: Option<HashMap<String, Value>>) -> Result<Box<dyn PlaceHolder>, Error> {
        Ok(Box::new(MessagePlaceholder {}))
    }
}

#[derive(Debug)]
pub struct MessagePlaceholder;


impl PlaceHolder for MessagePlaceholder {
    fn build_message<'a>(&self, record: &'a Record) -> &'a str {
        record.args().as_str().unwrap()
    }
}

pub struct LevelPlaceHolderBuilder;

impl PlaceHolderBuilder for LevelPlaceHolderBuilder {
    fn name(&self) -> String {
        "level".to_string()
    }

    fn build(&self, _value: Option<HashMap<String, Value>>) -> Result<Box<dyn PlaceHolder>, Error> {
        Ok(Box::new(LevelPlaceHolder))
    }
}

#[derive(Debug)]
pub struct LevelPlaceHolder;


impl PlaceHolder for LevelPlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> &'a str {
        record.level().as_str()
    }
}

pub struct ModulePlaceHolderBuilder;

impl PlaceHolderBuilder for ModulePlaceHolderBuilder {
    fn name(&self) -> String {
        "module".to_string()
    }

    fn build(&self, value: Option<HashMap<String, Value>>) -> Result<Box<dyn PlaceHolder>, Error> {
        if let Some(value) = value {
            if value.get("path-safe").and_then(|value| value.as_bool()).unwrap_or(false) {
                todo!("Path Safe is not implemented for this type")
            }
        }
        Ok(Box::new(ModulePlaceHolder {}))
    }
}

#[derive(Debug)]
pub struct ModulePlaceHolder;

impl PlaceHolder for ModulePlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> &'a str {
        record.module_path().unwrap_or("")
    }
}
