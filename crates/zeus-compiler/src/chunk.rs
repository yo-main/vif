use crate::constant::Constant;
use crate::op_code::OpCode;
use crate::CompilerError;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Constant>,
    pub lines: Vec<u64>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, chunk: OpCode, line: u64) {
        self.code.push(chunk);
        self.lines.push(line);
    }

    // Register a constant. The constant is stored in a Vec and the function returns it index
    pub fn add_constant(&mut self, constant: Constant) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Result<&Constant, CompilerError> {
        self.constants
            .get(index)
            .ok_or(CompilerError::ConstantNotFound)
    }

    pub fn get_line(&self, index: usize) -> u64 {
        *self.lines.get(index).unwrap_or(&0)
    }

    pub fn free_chunk(&mut self) {
        self.code.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<OpCode> {
        self.code.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<OpCode> {
        self.code.iter_mut()
    }
}
