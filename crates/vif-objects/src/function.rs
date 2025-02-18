use crate::chunk::Chunk;
use crate::variable::{InheritedVariable, Variable};

#[derive(Clone, Copy)]
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

    fn ne(&self, other: &usize) -> bool {
        match self {
            Self::Fixed(i) => i != other,
            Self::Infinite => false,
            Self::None => other != &0,
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
    pub locals: Vec<Variable>,
    pub inherited_locals: Vec<InheritedVariable>,
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
            inherited_locals: Vec::new(),
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

#[derive(Clone, Copy)]
pub enum NativeFunctionCallee {
    GetTime,
    Print,
    Sleep,
}

#[derive(Clone, Copy)]
pub struct NativeFunction {
    pub arity: Arity,
    pub name: &'static str,
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
                name: "get_time",
                function: callee,
            },
            NativeFunctionCallee::Print => Self {
                arity: Arity::Infinite,
                name: "printf",
                function: callee,
            },
            NativeFunctionCallee::Sleep => Self {
                arity: Arity::Fixed(1),
                name: "sleep",
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
