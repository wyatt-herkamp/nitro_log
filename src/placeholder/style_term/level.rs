use crate::Placeholder;
use log::{Level, Record};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Debug;
use style_term::{DefaultColor, StyleString, StylesContainer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelColorConfig {
    #[serde(default = "default_error")]
    pub error: StylesContainer,
    #[serde(default = "default_warn")]
    pub warn: StylesContainer,
    #[serde(default = "default_info")]
    pub info: StylesContainer,
    #[serde(default = "default_debug")]
    pub debug: StylesContainer,
    #[serde(default = "default_trace")]
    pub trace: StylesContainer,
}

fn default_error() -> StylesContainer {
    StylesContainer {
        text_color: Some(DefaultColor::Red.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_warn() -> StylesContainer {
    StylesContainer {
        text_color: Some(DefaultColor::BrightYellow.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_info() -> StylesContainer {
    StylesContainer {
        text_color: Some(DefaultColor::BrightGreen.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_debug() -> StylesContainer {
    StylesContainer {
        text_color: Some(DefaultColor::Gray.into()),
        background_color: None,
        styles: vec![],
    }
}

fn default_trace() -> StylesContainer {
    StylesContainer {
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
            Level::Error => "Error".apply_styles(&self.0.error).to_string(),
            Level::Warn => "Warn".apply_styles(&self.0.warn).to_string(),
            Level::Info => "Info".apply_styles(&self.0.info).to_string(),
            Level::Debug => "Debug".apply_styles(&self.0.debug).to_string(),
            Level::Trace => "Trace".apply_styles(&self.0.trace).to_string(),
        };
        Cow::Owned(value)
    }

    fn settings(&self) -> Option<Value> {
        let config = self.0.clone();
        serde_json::to_value(config).ok()
    }
}
