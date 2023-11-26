use zeus_compiler::Function;
use zeus_compiler::OpCode;

pub struct CallFrame<'stack, 'function>
where
    'function: 'stack,
{
    pub function: &'function Function,
    pub ip: std::slice::Iter<'stack, OpCode>,
}

impl<'stack, 'function> CallFrame<'stack, 'function>
where
    'function: 'stack,
{
    pub fn new(function: &'function Function, index: usize) -> Self {
        Self {
            function,
            ip: function.chunk.iter(index),
        }
    }

    pub fn reset_ip(&mut self, index: usize) {
        self.ip = self.function.chunk.iter(index);
    }
}
