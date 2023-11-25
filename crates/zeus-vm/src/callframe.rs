use zeus_compiler::Application;
use zeus_compiler::Function;
use zeus_compiler::OpCode;

pub struct CallFrame<'stack, 'function, R>
where
    'function: 'stack,
    R: CodeIterator,
{
    pub iterator: &'function R,
    start: usize,
    pub ip: std::slice::Iter<'stack, OpCode>,
}

impl<'stack, 'function, R> CallFrame<'stack, 'function, R>
where
    'function: 'stack,
    R: CodeIterator,
{
    pub fn new(iterator: &'function R, index: usize) -> Self {
        Self {
            iterator,
            ip: iterator.iter(0),
            start: index,
        }
    }

    pub fn reset_ip(&mut self, index: usize) {
        self.ip = self.iterator.iter(index);
    }
}

impl CodeIterator for Function {
    fn iter(&self, index: usize) -> std::slice::Iter<OpCode> {
        self.chunk.iter(index)
    }
}

impl CodeIterator for Application {
    fn iter(&self, index: usize) -> std::slice::Iter<OpCode> {
        self.chunk.iter(index)
    }
}

pub trait CodeIterator {
    fn iter(&self, index: usize) -> std::slice::Iter<OpCode>;
}
