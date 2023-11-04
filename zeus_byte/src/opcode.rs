use crate::error::ZeusError;
use crate::value::Constant;
use crate::value::Value;
use crate::value::Values;

pub enum OpCode<'c> {
    OP_RETURN,
    OP_CONSTANT(usize),
    OP_NEGATE,
    OP_ADD,
    OP_SUBSTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_MODULO,
    OP_TEST(&'c Value<'c>),
}

impl std::fmt::Display for OpCode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OP_RETURN => write!(f, "OP_RETURN"),
            Self::OP_CONSTANT(c) => write!(f, "OP_CONSTANT({c})"),
            Self::OP_NEGATE => write!(f, "OP_NEGATE"),
            Self::OP_ADD => write!(f, "OP_ADD"),
            Self::OP_SUBSTRACT => write!(f, "OP_SUBSTRACT"),
            Self::OP_MULTIPLY => write!(f, "OP_MULTIPLY"),
            Self::OP_DIVIDE => write!(f, "OP_DIVIDE"),
            Self::OP_MODULO => write!(f, "OP_MODULO"),
            Self::OP_TEST(_) => write!(f, "OP_TEST"),
        }
    }
}

pub struct Chunk<'c> {
    pub code: Vec<OpCode<'c>>,
    constants: Vec<Constant>,
    pub lines: Vec<i64>,
}

impl<'c> Chunk<'c> {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, chunk: OpCode<'c>, line: i64) {
        self.code.push(chunk);
        self.lines.push(line);
    }

    // Register a constant. The constant is stored in a Vec and the function returns it index
    pub fn add_constant(&mut self, constant: Constant) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Result<&Constant, ZeusError> {
        self.constants.get(index).ok_or(ZeusError::ValueNotFound)
    }

    pub fn get_line(&self, index: usize) -> i64 {
        *self.lines.get(index).unwrap_or(&-1)
    }

    pub fn free_chunk(&mut self) {
        self.code.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<'c, OpCode> {
        self.code.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'c, OpCode>
    where
        'a: 'c,
    {
        self.code.iter_mut()
    }
}
