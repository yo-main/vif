pub enum OpCode {
    OP_RETURN,
    OP_CONSTANT(usize),
    OP_NEGATE,
    OP_ADD,
    OP_SUBSTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_MODULO,
    OP_TRUE,
    OP_FALSE,
    OP_NONE,
    OP_NOT,
    OP_EQUAL,
    OP_NOT_EQUAL,
    OP_GREATER,
    OP_LESS,
    OP_GREATER_OR_EQUAL,
    OP_LESS_OR_EQUAL,
    OP_PRINT, // temp
    OP_POP,
}

impl std::fmt::Display for OpCode {
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
            Self::OP_TRUE => write!(f, "OP_TRUE"),
            Self::OP_FALSE => write!(f, "OP_FALSE"),
            Self::OP_NONE => write!(f, "OP_NONE"),
            Self::OP_NOT => write!(f, "OP_NOT"),
            Self::OP_EQUAL => write!(f, "OP_EQUAL"),
            Self::OP_NOT_EQUAL => write!(f, "OP_NOT_EQUAL"),
            Self::OP_GREATER => write!(f, "OP_GREATER"),
            Self::OP_LESS => write!(f, "OP_LESS"),
            Self::OP_GREATER_OR_EQUAL => write!(f, "OP_GREATER_OR_EQUAL"),
            Self::OP_LESS_OR_EQUAL => write!(f, "OP_LESS_OR_EQUAL"),
            Self::OP_PRINT => write!(f, "OP_PRINT"),
            Self::OP_POP => write!(f, "OP_POP"),
        }
    }
}
