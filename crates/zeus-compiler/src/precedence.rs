pub enum Precedence {
    None,
    Assignement,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    pub fn higher(&self) -> Self {
        match self {
            Self::None => Self::Assignement,
            Self::Assignement => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}

impl std::fmt::Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::Assignement => "Assignement",
                Self::Or => "Or",
                Self::And => "And",
                Self::Equality => "Equality",
                Self::Comparison => "Comparison",
                Self::Term => "Term",
                Self::Factor => "Factor",
                Self::Unary => "Unary",
                Self::Call => "Call",
                Self::Primary => "Primary",
            }
        )
    }
}
