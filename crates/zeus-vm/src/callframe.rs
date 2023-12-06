use zeus_compiler::Function;
use zeus_objects::op_code::OpCode;

pub struct CallFrame<'function> {
    pub function: &'function Function,
    pub ip: usize,
    pub stack_position: usize,
}

impl<'function> CallFrame<'function> {
    pub fn new(function: &'function Function, index: usize, stack_position: usize) -> Self {
        Self {
            function,
            ip: index,
            stack_position,
        }
    }

    pub fn next(&mut self) {
        self.ip += 1;
    }

    pub fn peek(&self) -> Option<&OpCode> {
        self.function.chunk.code.get(self.ip)
    }

    pub fn reset_ip(&mut self, index: usize) {
        self.ip = index;
    }

    pub fn advance_by(&mut self, index: usize) {
        self.ip += index;
    }
}
