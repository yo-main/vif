pub enum OpCode {
    OP_RETURN,
    OP_CONSTANT(usize),
    OP_NEGATE,
    OP_ADD,
    OP_SUBSTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_MODULO,
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
        }
    }
}
