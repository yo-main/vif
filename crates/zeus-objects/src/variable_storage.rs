use crate::value::Value;

pub struct VariableStore<'globals, 'value> {
    storage: Vec<KV<'globals, 'value>>,
}

#[derive(Debug)]
struct KV<'globals, 'value> {
    key: &'globals str,
    value: Box<Value<'value>>,
}

impl PartialOrd for KV<'_, '_> {
    fn lt(&self, other: &Self) -> bool {
        self.key.lt(other.key)
    }

    fn le(&self, other: &Self) -> bool {
        self.key.le(other.key)
    }

    fn gt(&self, other: &Self) -> bool {
        self.key.gt(other.key)
    }

    fn ge(&self, other: &Self) -> bool {
        self.key.ge(other.key)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(other.key)
    }
}

impl PartialEq for KV<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}

impl Ord for KV<'_, '_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(other.key)
    }
}

impl<'globals, 'value> KV<'globals, 'value> {
    fn new(key: &'globals str, value: Value<'value>) -> Self {
        Self {
            key,
            value: Box::new(value),
        }
    }
}

impl Eq for KV<'_, '_> {}

impl<'globals, 'value, 'variables> VariableStore<'globals, 'value> {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: &'globals str, value: Value<'value>) -> bool {
        let new = KV::new(key, value);
        match self.storage.iter_mut().find(|v| v.key.eq(new.key)) {
            Some(v) => {
                *v.value = *new.value;
                return true;
            }
            None => {
                self.storage.push(new);
                return false;
            }
        }
    }

    pub fn get(&self, key: &'globals str) -> &Value<'value> {
        &self.storage.iter().find(|&v| v.key.eq(key)).unwrap().value
    }
}
