use std::borrow::Cow;
use std::fmt::{Debug};
use log::{error, info, Record, warn};

use nitro_log::{LoggerBuilders, NitroLogger};

use std::path::PathBuf;




use serde_json::Value;
use nitro_log::error::Error;


use nitro_log::placeholder::{Placeholder, PlaceholderBuilder};

pub struct MyPlaceHolderBuilder;

impl PlaceholderBuilder for MyPlaceHolderBuilder {
    /// Give it a name
    fn name<'a>(&self) -> &'a str {
        "myPlaceHolder"
    }

    fn build(&self, _value: Option<Value>) -> Result<Box<dyn Placeholder>, Error> {
        Ok(Box::new(MyPlaceHolderTest {}))
    }
}

#[derive(Debug)]
pub struct MyPlaceHolderTest;


impl Placeholder for MyPlaceHolderTest {
    fn build_message<'a>(&self, record: &'a Record) -> Cow<'a, str> {
        Cow::Owned(record.line().map(|l| l.to_string()).unwrap_or_else(|| "NOT_FOUND".to_string()))
    }

    /// Return the settings. Currently the backend usage of this is not added
    fn settings(&self) -> Option<Value> {
        None
    }
}

#[test]
fn test() {
// By using the LoggerBuilders::default you get all the default Placeholders and Targets
    let mut builders = LoggerBuilders::default();
    builders.placeholders.push(Box::new(MyPlaceHolderBuilder {}));
    NitroLogger::load_file(PathBuf::from("tests/custom_placeholder.json"), builders).unwrap();
    info!("INFO HEY");
    warn!("Warn HEY");
    error!("Error HEY");
}