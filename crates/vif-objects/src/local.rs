use crate::variable::Variable;

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

#[derive(Clone)]
pub struct InheritedLocal {
    pub var_name: Box<String>,
    pub depth: usize,
    pub pos: usize,
}

#[derive(PartialEq, Debug)]
pub struct InheritedLocalPos {
    pub pos: usize,
    pub depth: usize,
}

pub enum VariableType {
    None,
    Local(usize),
    Inherited(InheritedLocalPos),
}

impl std::fmt::Display for InheritedLocalPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={}, depth={}", self.pos, self.depth)
    }
}
