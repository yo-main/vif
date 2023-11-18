#[derive(Debug)]
pub enum Variable {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::String(i) => write!(f, "{}", i),
            Self::Identifier(i) => write!(f, "{}", i),
        }
    }
}
