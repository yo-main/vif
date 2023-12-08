use crate::value::Value;
use std::collections::HashMap;

pub struct VariableStore<'globals, 'value> {
    storage: HashMap<&'globals str, Value<'value>>,
}

impl<'globals, 'value, 'variables> VariableStore<'globals, 'value> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &'globals str, value: Value<'value>) -> Option<Value<'_>> {
        self.storage.insert(key, value)
    }

    pub fn get(&self, key: &'globals str) -> Option<&Value<'value>> {
        self.storage.get(key)
    }
}
