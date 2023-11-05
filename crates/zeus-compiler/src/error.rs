#[derive(Debug)]
pub enum CompilerError {
    ConstantNotFound,
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compiler error")
    }
}
