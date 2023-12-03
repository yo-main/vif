use crate::local::Local;
use crate::Chunk;

pub struct Function {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
    pub locals: Vec<Local>,
}

impl Function {
    pub fn new(arity: usize, name: String) -> Self {
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

#[derive(Clone)]
pub enum NativeFunctionCallee {
    GetTime,
}

#[derive(Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub name: String,
    pub function: NativeFunctionCallee,
}

impl NativeFunction {
    pub fn new(callee: NativeFunctionCallee) -> Self {
        match callee {
            NativeFunctionCallee::GetTime => Self {
                arity: 0,
                name: "get_time".to_owned(),
                function: callee,
            },
        }
    }
}

impl std::fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
