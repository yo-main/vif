use std::ops::RangeFrom;

use crate::value::Value;

pub struct Stack<'value> {
    stack: Vec<Value<'value>>,
}

impl<'value> Stack<'value> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<Value<'value>> {
        self.stack.pop()
    }

    pub fn drop_last(&mut self) {
        self.stack.pop();
        // unsafe { self.stack.set_len(self.stack.len() - 1) }
    }

    pub fn push(&mut self, value: Value<'value>) {
        self.stack.push(value)
    }

    pub fn set(&mut self, n: usize, value: Value<'value>) {
        self.stack.push(value)
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn peek(&self, n: usize) -> &Value<'value> {
        &self.stack[n]
    }

    pub fn peek_last(&self) -> &Value<'value> {
        self.stack.last().unwrap()
    }

    pub fn peek_last_mut(&mut self) -> &mut Value<'value> {
        self.stack.last_mut().unwrap()
    }

    pub fn get_slice(&self, range: RangeFrom<usize>) -> &[Value<'value>] {
        &self.stack[range]
    }

    pub fn truncate(&mut self, n: usize) {
        self.stack.truncate(n)
    }
}

impl std::fmt::Display for Stack<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.stack)
    }
}

impl std::fmt::Debug for Stack<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.stack)
    }
}
