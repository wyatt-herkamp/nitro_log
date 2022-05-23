use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::path::{MAIN_SEPARATOR, PathBuf};
use log::Record;
use serde_json::Value;
use crate::{Error, PlaceHolder};
use crate::placeholder::PlaceHolderBuilder;

pub struct MessagePlaceholderBuilder;

impl PlaceHolderBuilder for MessagePlaceholderBuilder {
    fn name<'a>(&self) -> &'a str {
        "message"
    }

    fn build(&self, _value: Option<Value>) -> Result<Box<dyn PlaceHolder>, Error> {
        Ok(Box::new(MessagePlaceholder {}))
    }
}

#[derive(Debug)]
pub struct MessagePlaceholder;


impl PlaceHolder for MessagePlaceholder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Borrowed( record.args().as_str().unwrap())

    }

    fn settings(&self) -> Option<Value> {
        None
    }
}

pub struct LevelPlaceHolderBuilder;

impl PlaceHolderBuilder for LevelPlaceHolderBuilder {
    fn name<'a>(&self) -> &'a str {
        "level"
    }


    fn build(&self, _value: Option<Value>) -> Result<Box<dyn PlaceHolder>, Error> {
        Ok(Box::new(LevelPlaceHolder))
    }
}

#[derive(Debug)]
pub struct LevelPlaceHolder;


impl PlaceHolder for LevelPlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Borrowed(record.level().as_str())
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}

pub struct ModulePlaceHolderBuilder;

impl PlaceHolderBuilder for ModulePlaceHolderBuilder {
    fn name<'a>(&self) -> &'a str {
        "module"
    }


    fn build(&self, value: Option<Value>) -> Result<Box<dyn PlaceHolder>, Error> {
        if let Some(value) = value {
            if value.get("path-safe").and_then(|value| value.as_bool()).unwrap_or(false) {
                return Ok(Box::new(PathModulePlaceHolder {}));
            }
        }
        Ok(Box::new(ModulePlaceHolder {}))
    }
}

#[derive(Debug)]
pub struct ModulePlaceHolder;


impl PlaceHolder for ModulePlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Borrowed(record.module_path().unwrap_or(""))
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}

#[derive(Debug)]
pub struct PathModulePlaceHolder;


impl PlaceHolder for PathModulePlaceHolder {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Owned(record.module_path().unwrap_or("").replace("::", &MAIN_SEPARATOR.to_string()))
    }
    fn settings(&self) -> Option<Value> {
        None
    }
}
