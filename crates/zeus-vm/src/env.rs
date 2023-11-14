use crate::value::Value;
use std::collections::HashMap;

pub struct Env<'a, 'b> {
    variables: HashMap<&'a str, Value<'b>>,
}

impl Env<'_, '_> {
    pub fn new() -> Self {
        Env {
            variables: HashMap::new(),
        }
    }
}
