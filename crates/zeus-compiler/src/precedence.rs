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
