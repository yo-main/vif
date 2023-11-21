use crate::Chunk;

pub struct Function {
    pub arity: u8,
    pub chunk: Chunk,
    pub name: String,
}

impl Function {
    pub fn new(arity: u8, name: String) -> Self {
        Self {
            arity,
            name,
            chunk: Chunk::new(),
        }
    }
}
