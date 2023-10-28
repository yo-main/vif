use crate::ast::{BuiltIn, Value};
use crate::errors::ZeusErrorType;
use std::collections::HashMap;

pub struct Environment {
    stack: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        let global = HashMap::from([
            ("print".to_owned(), Value::BuiltIn(BuiltIn::Print)),
            ("get_time".to_owned(), Value::BuiltIn(BuiltIn::GetTime)),
        ]);

        Environment {
            stack: vec![global],
        }
    }

    pub fn start_new(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn close(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, name: String, value: Value) -> Option<Value> {
        let values = self.stack.last_mut().unwrap();
        values.insert(name, value)
    }

    pub fn get(&self, name: &str) -> Result<&Value, ZeusErrorType> {
        for values in self.stack.iter().rev() {
            if let Some(value) = values.get(name) {
                return Ok(value);
            }
        }

        return Err(ZeusErrorType::UnassignedVariable(name.to_owned()));
    }

    pub fn update(&mut self, name: &str, value: Value) -> Result<(), ZeusErrorType> {
        for values in self.stack.iter_mut().rev() {
            if let Some(item) = values.get_mut(name) {
                *item = value;
                return Ok(());
            }
        }

        return Err(ZeusErrorType::InterpreterError(format!(
            "Can't assign - variable does not exit: {}",
            name
        )));
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.stack.last().unwrap().keys())
    }
}
