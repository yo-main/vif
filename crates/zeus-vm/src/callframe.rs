use crate::value::Value;
use zeus_compiler::{Function, OpCode};

pub struct CallFrame<'stack, 'function, 'value>
where
    'function: 'stack,
{
    pub function: &'function Function,
    pub stack: &'stack mut Vec<Value<'value>>,
    start: usize,
    pub ip: std::slice::Iter<'stack, OpCode>,
}

impl<'stack, 'function, 'value> CallFrame<'stack, 'function, 'value>
where
    'function: 'stack,
{
    pub fn new(
        function: &'function Function,
        stack: &'stack mut Vec<Value<'value>>,
        index: usize,
    ) -> Self {
        Self {
            function,
            ip: function.chunk.iter(0),
            stack,
            start: index,
        }
    }
}

impl<'stack, 'function, 'value> CallFrame<'stack, 'function, 'value> {
    pub fn reset_ip(&mut self, index: usize) {
        self.ip = self.function.chunk.iter(index)
    }
}
