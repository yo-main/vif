use crate::function::Function;
use crate::function::NativeFunction;

#[derive(Debug, PartialEq)]
pub enum Global {
    Integer(i64),
    Float(f64),
    String(Box<String>),
    Identifier(Box<String>),
    Function(Box<Function>),
    Native(NativeFunction),
}

impl std::fmt::Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::String(i) => write!(f, "{}", i),
            Self::Identifier(i) => write!(f, "{}", i),
            Self::Function(func) => write!(f, "{}", func.name),
            Self::Native(func) => write!(f, "{}", func.name),
        }
    }
}
