#[derive(Debug)]
pub enum CompilerError {
    ConstantNotFound,
    ScanningError(String),
    SyntaxError(String),
    Unknown(String),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compiler error")
    }
}
