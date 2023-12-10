use crate::op_code::OpCode;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<u64>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, op_code: OpCode, line: u64) {
        self.code.push(op_code);
        self.lines.push(line);
    }

    pub fn get_line(&self, index: usize) -> u64 {
        *self.lines.get(index).unwrap_or(&0)
    }

    pub fn iter(&self, index: usize) -> std::slice::Iter<OpCode> {
        self.code[index..].iter()
    }
}
