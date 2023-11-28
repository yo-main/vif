use crate::op_code::OpCode;

pub struct Chunk {
    pub code: Vec<OpCode>,
    // pub constants: Vec<Variable>,
    pub lines: Vec<u64>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            // constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, op_code: OpCode, line: u64) {
        self.code.push(op_code);
        self.lines.push(line);
    }

    // Register a constant. The constant is stored in a Vec and the function returns it index
    // pub fn add_constant(&mut self, constant: Variable) -> usize {
    //     self.constants.push(constant);
    //     self.constants.len() - 1
    // }

    // pub fn get_constant(&self, index: usize) -> Result<&Variable, CompilerError> {
    //     self.constants
    //         .get(index)
    //         .ok_or(CompilerError::ConstantNotFound)
    // }

    pub fn get_line(&self, index: usize) -> u64 {
        *self.lines.get(index).unwrap_or(&0)
    }

    pub fn free_chunk(&mut self) {
        self.code.clear();
    }

    pub fn iter(&self, index: usize) -> std::slice::Iter<OpCode> {
        self.code[index..].iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<OpCode> {
        self.code.iter_mut()
    }
}
