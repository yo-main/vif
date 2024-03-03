pub struct Variable {
    pub name: Box<String>,
    pub depth: Option<usize>,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.depth == other.depth
    }
}

impl Variable {
    pub fn new(name: Box<String>, depth: Option<usize>) -> Self {
        Self { name, depth }
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "local[{}]{}", self.name, self.depth.unwrap_or(0))
    }
}

impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
pub struct InheritedVariable {
    pub var_name: Box<String>,
    pub depth: usize,
    pub pos: usize,
}

#[derive(PartialEq)]
pub struct InheritedLocalPos {
    pub pos: usize,
    pub depth: usize,
}

pub enum VariableType {
    Local(usize),
    Inherited(InheritedLocalPos),
    Global(usize),
}

impl std::fmt::Display for InheritedLocalPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={}, depth={}", self.pos, self.depth)
    }
}

impl std::fmt::Debug for InheritedLocalPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={}, depth={}", self.pos, self.depth)
    }
}

impl std::fmt::Debug for InheritedVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.var_name)
    }
}
