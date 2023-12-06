use crate::chunk::Chunk;
use crate::local::Local;

#[derive(Clone)]
pub enum Arity {
    Fixed(usize),
    Infinite,
    None,
}

impl std::ops::AddAssign<usize> for Arity {
    fn add_assign(&mut self, rhs: usize) {
        match self {
            Arity::None => *self = Arity::Fixed(rhs),
            Arity::Fixed(ref mut i) => *i += rhs,
            Arity::Infinite => (),
        }
    }
}

impl PartialEq<usize> for Arity {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Self::Fixed(i) => i == other,
            Self::Infinite => true,
            Self::None => other == &0,
        }
    }
}

impl std::fmt::Display for Arity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(i) => write!(f, "{i}"),
            Self::Infinite => write!(f, "Infinite"),
            Self::None => write!(f, "None"),
        }
    }
}

impl std::fmt::Debug for Arity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(i) => write!(f, "{i}"),
            Self::Infinite => write!(f, "Infinite"),
            Self::None => write!(f, "None"),
        }
    }
}

pub struct Function {
    pub arity: Arity,
    pub chunk: Chunk,
    pub name: String,
    pub locals: Vec<Local>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name // TODO: this will have to be reworked
    }
}

impl Function {
    pub fn new(arity: Arity, name: String) -> Self {
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
    Print,
}

#[derive(Clone)]
pub struct NativeFunction {
    pub arity: Arity,
    pub name: String,
    pub function: NativeFunctionCallee,
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl NativeFunction {
    pub fn new(callee: NativeFunctionCallee) -> Self {
        match callee {
            NativeFunctionCallee::GetTime => Self {
                arity: Arity::None,
                name: "get_time".to_owned(),
                function: callee,
            },
            NativeFunctionCallee::Print => Self {
                arity: Arity::Infinite,
                name: "print".to_owned(),
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
