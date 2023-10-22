use crate::ast::Value;
use crate::errors::ZeusErrorType;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) -> Option<Value> {
        self.values.insert(name, value)
    }

    pub fn get(&self, name: &str) -> Result<&Value, ZeusErrorType> {
        self.values
            .get(name)
            .ok_or_else(|| ZeusErrorType::UnassignedVariable(name.to_owned()))
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.values.keys())
    }
}
