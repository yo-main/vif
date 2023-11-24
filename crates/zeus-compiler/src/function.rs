use crate::local::Local;
use crate::Chunk;

pub struct Function {
    pub arity: u8,
    pub chunk: Chunk,
    pub name: String,
    pub locals: Vec<Local>,
}

impl Function {
    pub fn new(arity: u8, name: String) -> Self {
        Self {
            arity,
            name,
            chunk: Chunk::new(),
            locals: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.chunk.code.len()
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
