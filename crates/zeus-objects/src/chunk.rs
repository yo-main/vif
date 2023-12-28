use crate::op_code::OpCode;

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    pub fn write_chunk(&mut self, op_code: OpCode) {
        self.code.push(op_code);
    }

    pub fn get_line(&self, index: usize) -> u64 {
        0
    }

    pub fn iter(&self, index: usize) -> std::slice::Iter<OpCode> {
        self.code[index..].iter()
    }
}
