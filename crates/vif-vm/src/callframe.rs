use vif_objects::function::Function;
use vif_objects::op_code::OpCode;

pub struct CallFrame<'stack, 'function>
where
    'function: 'stack,
{
    function: &'function Function,
    ip: std::slice::Iter<'stack, OpCode>,
    stack_position: usize,
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

    pub fn get_position(&self) -> usize {
        self.stack_position
    }

    pub fn get_function_name(&self) -> &str {
        self.function.name.as_str()
    }
}

impl<'stack, 'function> std::iter::Iterator for CallFrame<'stack, 'function> {
    type Item = &'stack OpCode;

    fn next(&mut self) -> Option<Self::Item> {
        self.ip.next()
    }

    fn advance_by(&mut self, n: usize) -> Result<(), std::num::NonZeroUsize> {
        self.ip.advance_by(n)
    }
}
