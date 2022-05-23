
use std::io::Write;

use log::kv::{Error, Key, Value, Visitor};


use crate::loggers::LoggerWriter;

pub struct DefaultStructureDump<'a> {
    pub write: Vec<LoggerWriter<'a>>,
}

impl<'kvs, 'a> Visitor<'kvs> for DefaultStructureDump<'a> {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), Error> {
        for writer in self.write.iter_mut() {
            writer.writer.write_all("\n".as_bytes())?;
            writer.writer.write_all(format!("{key}: {value}").as_bytes())?;
        }
        Ok(())
    }
}
