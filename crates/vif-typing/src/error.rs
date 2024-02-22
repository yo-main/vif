pub enum TypingError {
    Mutability(String),
}

impl std::fmt::Display for TypingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mutability(s) => write!(f, "{}", s),
        }
    }
}

impl std::fmt::Debug for TypingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mutability(s) => write!(f, "{}", s),
        }
    }
}
