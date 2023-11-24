use zeus_compiler::{Function, OpCode};

pub struct CallFrame<'stack, 'function>
where
    'function: 'stack,
{
    pub function: &'function Function,
    start: usize,
    pub ip: std::slice::Iter<'stack, OpCode>,
}

impl<'stack, 'function, 'value> CallFrame<'stack, 'function>
where
    'function: 'stack,
{
    pub fn new(function: &'function Function, index: usize) -> Self {
        Self {
            function,
            ip: function.chunk.iter(0),
            start: index,
        }
    }
}

impl<'stack, 'function> CallFrame<'stack, 'function> {
    pub fn reset_ip(&mut self, index: usize) {
        self.ip = self.function.chunk.iter(index)
    }
}
