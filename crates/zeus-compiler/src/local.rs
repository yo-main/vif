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
