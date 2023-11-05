use zeus_compiler::Constant;

#[derive(Debug)]
pub enum Value<'c> {
    Integer(i64),
    Index(i64),
    Constant(&'c Constant),
    BinaryOp(BinaryOp),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Index(i) => write!(f, "{}", i),
            Self::Constant(c) => write!(f, "{}", *c),
            Self::BinaryOp(o) => write!(f, "{}", o),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Substract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Modulo => write!(f, "%"),
        }
    }
}

#[derive(Debug)]
pub struct Values<'c> {
    values: Vec<Value<'c>>,
}
impl<'c> Values<'c> {
    pub fn new() -> Self {
        Values { values: Vec::new() }
    }

    pub fn add(&mut self, value: Value<'c>) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn get(&self, index: usize) -> Option<&Value<'c>> {
        self.values.get(index)
    }

    pub fn last(&self) -> Option<&Value<'c>> {
        self.values.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut Value<'c>> {
        self.values.last_mut()
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.values.iter()
    }
}
