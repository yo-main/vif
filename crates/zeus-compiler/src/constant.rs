#[derive(Debug)]
pub enum Constant {
    Integer(i64),
    String(String),
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::String(i) => write!(f, "{}", i),
        }
    }
}
