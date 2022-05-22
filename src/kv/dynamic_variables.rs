use log::kv::{Error, Key, ToValue, Value, Visitor};

pub struct DynamicVariableVisitor<'a, 'kvs>{
    pub variable_name: &'a str,
    pub option: Option<Value<'kvs>>
}

impl<'kvs, 'a> Visitor<'kvs> for DynamicVariableVisitor<'a,'kvs>{
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), Error> {
        if key.as_str().eq(self.variable_name) {
           self.option = Some(value)
        }
        Ok(())
    }
}