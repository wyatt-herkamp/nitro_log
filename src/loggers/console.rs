use std::env::var;
use std::io::{stdout, Stdout, Write};
use log::Record;
use serde::{Deserialize, Serialize};

use crate::loggers::{Logger, LoggerTarget};
use crate::{Error, NitroLogger, PlaceHolders};
use crate::format::{Format, FormatError, FormatSection};

pub struct ConsoleLogger {
    pub format: Format,
    pub console: Stdout,
}

impl ConsoleLogger {
    pub fn init(placeholders: &PlaceHolders, config: ConsoleConfig) -> Result<ConsoleLogger, Error> {
        let logger = ConsoleLogger {
            format: Format::new(placeholders, &config.format)?,
            console: stdout(),
        };
        return Ok(logger);
    }
}


impl LoggerTarget for ConsoleLogger {
    fn log(
        &self,
        record: &Record,
    ) -> Result<(), Error> {
        let mut out = stdout().lock();
        for x in self.format.format {
            match x {
                FormatSection::Text(value) => {
                    out.write_all(value.as_bytes())?;
                }
                FormatSection::Variable(variable) => {
                    out.write_all(format!("Coming Soon {}", variable).as_bytes())?;
                }
                FormatSection::Placeholder(placeholder) => {
                    out.write_all(placeholder.build_message(&record).as_bytes())?;
                }
            }
        }
        out.write_all("\n".as_bytes())?;
        out.flush()?;
        Ok(())
    }

    fn name(&self) -> String {
        return "console".to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConsoleConfig {
    pub format: String,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        return ConsoleConfig {
            format: "{{module()}} {{level()}}: {{message()}}".to_string(),
        };
    }
}
