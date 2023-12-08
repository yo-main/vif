use crate::value::Value;
use std::collections::HashMap;

pub struct VariableStore<'globals, 'value> {
    // storage: HashMap<&'globals str, Value<'value>>,
    storage: Vec<(&'globals str, Value<'value>)>,
}

impl<'globals, 'value, 'variables> VariableStore<'globals, 'value> {
    pub fn new() -> Self {
        Self {
            // storage: HashMap::with_capacity(1000),
            storage: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: &'globals str, value: Value<'value>) -> bool {
        // self.storage.insert(key, value)
        match self.storage.iter_mut().find(|v| v.0 == key) {
            Some(v) => {
                *v = (key, value);
                return true;
            }
            None => {
                self.storage.push((key, value));
                return false;
            }
        }
    }

    pub fn get(&self, key: &'globals str) -> &Value<'value> {
        // &self.storage[key]
        match self.storage.iter().find(|v| v.0 == key) {
            Some(v) => &v.1,
            None => panic!("No"),
        }
    }
}