use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use log::{Level, Record};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use style_term::{Color, DefaultColor, Styles, StyleString};
use crate::Placeholder;
use crate::placeholder::standard_placeholders::LevelPlaceholderSettings;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelColorConfig {
    #[serde(default = "default_error")]
    pub error: Styles,
    #[serde(default = "default_warn")]
    pub warn: Styles,
    #[serde(default = "default_info")]
    pub info: Styles,
    #[serde(default = "default_debug")]
    pub debug: Styles,
    #[serde(default = "default_trace")]
    pub trace: Styles,
}

fn default_error() -> Styles {
    Styles {
        text_color: Some(DefaultColor::Red.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_warn() -> Styles {
    Styles {
        text_color: Some(DefaultColor::BrightYellow.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_info() -> Styles {
    Styles {
        text_color: Some(DefaultColor::BrightGreen.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_debug() -> Styles {
    Styles {
        text_color: Some(DefaultColor::Gray.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_trace() -> Styles {
    Styles {
        text_color: Some(DefaultColor::BrightBlue.into()),
        background_color: None,
        styles: vec![],
    }
}

#[derive(Debug)]
pub struct StyledLevelPlaceholder(LevelColorConfig);

impl From<LevelColorConfig> for StyledLevelPlaceholder {
    fn from(value: LevelColorConfig) -> Self {
        Self(value)
    }
}


impl Placeholder for StyledLevelPlaceholder {
    fn build_message<'message>(&'message self, record: &'message Record) -> Cow<'message, str> {
        let value = match record.level() {
            Level::Error => { "Error".apply_styles(&self.0.error).to_string() }
            Level::Warn => { "Warn".apply_styles(&self.0.warn).to_string() }
            Level::Info => { "Info".apply_styles(&self.0.info).to_string() }
            Level::Debug => { "Debug".apply_styles(&self.0.debug).to_string() }
            Level::Trace => { "Trace".apply_styles(&self.0.trace).to_string() }
        };
        Cow::Owned(value)
    }

    fn settings(&self) -> Option<Value> {
        let config = self.0.clone();
        serde_json::to_value(config).ok()
    }
}