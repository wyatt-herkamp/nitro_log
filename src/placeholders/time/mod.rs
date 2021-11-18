use crate::{Logger, Placeholder};
use chrono::{DateTime, Local};
use log::Record;
use std::collections::HashMap;

pub struct DateTimePlaceholder;

static DEFAULT: &str = "%Y-%m-%d %H:%M:%S";

impl Placeholder for DateTimePlaceholder {
    fn replace(
        &self,
        properties: HashMap<String, String>,
        record: &Record,
        logger: &Logger,
    ) -> Option<String> {
        let time = DEFAULT.to_string();
        let x = properties.get("format").unwrap_or(&time).replace("$", "%");
        Some(Local::now().format(&x).to_string())
    }

    fn name(&self) -> &'static str {
        return "datetime";
    }
}
