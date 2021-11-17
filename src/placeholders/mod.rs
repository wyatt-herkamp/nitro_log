use std::collections::HashMap;
use log::Record;
use crate::Logger;

pub type Placeholders = Vec<Box<dyn Placeholder>>;

pub trait FindPlaceholder {
    fn get_placeholder(&self, name: String) -> Option<&Box<dyn Placeholder>>;
}

impl FindPlaceholder for Placeholders {
    fn get_placeholder(&self, name: String) -> Option<&Box<dyn Placeholder>> {
        for x in self {
            if name.eq(x.name()) {
                return Some(x);
            }
        }
        return None;
    }
}
// %.+?%
/// %module% %date_time{format=''}% %level%: %message%
pub trait Placeholder: Sync + Send  {
    fn replace(&self, properties: HashMap<String, String>, record: &Record, logger: &Logger) -> String;
    fn name(&self,) -> &'static str;
}

pub struct ModulePlaceHolder;

impl Placeholder for ModulePlaceHolder {
    fn replace(&self,properties: HashMap<String, String>, record: &Record, logger: &Logger) -> String {
        record.module_path().unwrap().to_string()
    }

    fn name(&self,) -> &'static str {
        return "module";
    }
}

pub struct LevelPlaceholder;

impl Placeholder for LevelPlaceholder {
    fn replace(&self,properties: HashMap<String, String>, record: &Record, logger: &Logger) -> String {
        record.metadata().level().to_string()
    }

    fn name(&self) -> &'static str {
        return "level";
    }
}

pub struct MessagePlaceholder;

impl Placeholder for MessagePlaceholder {
    fn replace(&self,properties: HashMap<String, String>, record: &Record, logger: &Logger) -> String {
        record.args().to_string()
    }

    fn name(&self) -> &'static str {
        return "message";
    }
}