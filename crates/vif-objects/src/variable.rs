#[derive(Debug)]
pub struct Variable {
    pub name: Box<String>,
    pub depth: Option<usize>,
    pub mutable: bool,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.depth == other.depth
    }
}

impl Variable {
    pub fn new(name: Box<String>, depth: Option<usize>, mutable: bool) -> Self {
        Self {
            name,
            depth,
            mutable,
        }
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "local name={} depth={}",
            self.name,
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
