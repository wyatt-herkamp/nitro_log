use std::collections::HashMap;
use std::fmt::{Display, Formatter};


use regex::Regex;
use serde_json::Value;
use thiserror::Error;
use crate::{Placeholder};
use crate::config::FormatConfig;
use crate::placeholder::PlaceholderBuilder;

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("{0}")]
    SettingParseError(serde_json::Error),
    #[error("Missing Key {0}")]
    MissingKey(String),

}

#[derive(Debug)]
pub struct Format {
    pub format: Vec<FormatSection>,
}

#[derive(Debug)]
pub enum FormatSection {
    Text(String),
    Variable(String),
    Placeholder(Box<dyn Placeholder>),
}

impl Format {
    /// {{ placeholder({"format": "", "key": ""}) }}
    /// {{ variable.name }}
    /// Example format `Important Log Message Here  {{level({"color": true })}} {{ repository.name }}: {{message({})}}!!!`
    pub fn new(placeholders: &Vec<Box<dyn PlaceholderBuilder>>, format: FormatConfig, path_safe: bool) -> Result<Format, FormatError> where {
        let special_call_parse: Regex = Regex::new("\\{\\{(?P<key>.+?)(?P<PlaceHolder>[(](?P<settings>.+?)?[)])?}}+").unwrap();
        let mut matches = special_call_parse.captures_iter(format.format.as_str());
        let mut variables = Vec::new();
        for value in special_call_parse.split(format.format.as_str()) {
            variables.push(FormatSection::Text(value.to_string()));
            if let Some(capture) = matches.next() {
                let key = capture.name("key").ok_or_else(|| FormatError::MissingKey("Missing Key".to_string()))?;
                let special_call = if capture.name("PlaceHolder").is_some() {
                    let settings = if let Some(settings) = capture.name("settings") {
                        let settings_string = settings.as_str();
                        let mut placeholder_settings = if settings_string.starts_with("{") && settings_string.ends_with("}") {
                            serde_json::from_str(&settings_string).map_err(FormatError::SettingParseError)
                        } else {
                            format.placeholders.get(settings_string).ok_or(FormatError::MissingKey(format!("Missing Setting for {}", settings_string))).cloned()
                        }?;
                        let value_map = placeholder_settings.as_object_mut().unwrap();
                        value_map.insert("path".to_string(), Value::Bool(path_safe.clone()));
                        Some(serde_json::to_value(placeholder_settings).map_err(FormatError::SettingParseError)?)
                    } else {
                        None
                    };
                    if let Some(placeholder) = placeholders.iter().find(|pb| pb.name().eq(key.as_str())) {
                        FormatSection::Placeholder(placeholder.build(settings).unwrap())
                    } else {
                        continue;
                    }
                } else {
                    FormatSection::Variable(key.as_str().trim().to_string())
                };

                variables.push(special_call);
            }
        }
        Ok(Format {
            format: variables
        })
    }
}

