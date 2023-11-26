use crate::Variable;

pub struct Local {
    pub variable: Variable,
    pub depth: Option<usize>,
}

impl Local {
    pub fn new(variable: Variable, depth: Option<usize>) -> Self {
        Self { variable, depth }
    }
}

impl std::fmt::Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "local {} {}", self.variable, self.depth.is_some())
    }
}
