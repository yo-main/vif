use crate::function::Function;
use crate::function::NativeFunction;
use crate::variable::Variable;

#[derive(Debug, PartialEq)]
pub enum Global {
    Integer(i64),
    Float(f64),
    String(Box<String>),
    Identifier(Variable),
    Function(Box<Function>),
    Native(NativeFunction),
}

impl std::fmt::Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "Integer({})", i),
            Self::Float(i) => write!(f, "Float({})", i),
            Self::String(i) => write!(f, "String({})", i),
            Self::Identifier(i) => write!(f, "Identifier({})", i),
            Self::Function(func) => write!(f, "function({})", func.name),
            Self::Native(func) => write!(f, "builtin({})", func.name),
        }
    }
}
