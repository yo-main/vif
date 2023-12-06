use zeus_compiler::Function;
use zeus_objects::op_code::OpCode;

pub struct CallFrame<'stack, 'function>
where
    'function: 'stack,
{
    pub function: &'function Function,
    pub ip: std::slice::Iter<'stack, OpCode>,
    pub stack_position: usize,
}

impl<'stack, 'function> CallFrame<'stack, 'function>
where
    'function: 'stack,
{
    pub fn new(function: &'function Function, index: usize, stack_position: usize) -> Self {
        Self {
            function,
            ip: function.chunk.iter(index),
            stack_position,
        }
    }

    pub fn reset_ip(&mut self, index: usize) {
        self.ip = self.function.chunk.iter(index);
    }
}
