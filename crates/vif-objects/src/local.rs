pub struct Local {
    pub variable: Box<String>,
    pub depth: Option<usize>,
}

impl Local {
    pub fn new(variable: Box<String>, depth: Option<usize>) -> Self {
        Self { variable, depth }
    }
}

impl std::fmt::Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "local name={} depth={}",
            self.variable,
            self.depth.unwrap_or(0)
        )
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
