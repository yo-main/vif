use crate::value::Value;

pub struct VariableStore<'globals, 'value> {
    storage: Vec<(&'globals str, Box<Value<'value>>)>,
}

impl<'globals, 'value, 'variables> VariableStore<'globals, 'value> {
    pub fn new() -> Self {
        Self {
            storage: Vec::with_capacity(1000),
        }
    }

    pub fn insert(&mut self, key: &'globals str, value: Value<'value>) -> bool {
        match self.storage.iter_mut().find(|v| v.0 == key) {
            Some(v) => {
                *v = (key, Box::new(value));
                return true;
            }
            None => {
                self.storage.push((key, Box::new(value)));
                return false;
            }
        }
    }

    pub fn get(&self, key: &'globals str) -> &Value<'value> {
        &self
            .storage
            .iter()
            .rev()
            .find(|(v, _)| v == &key)
            .unwrap()
            .1
    }
}
