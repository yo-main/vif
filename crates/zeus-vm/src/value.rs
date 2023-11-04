use crate::error::ZeusError;

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

#[derive(Debug)]
pub enum Constant {
    Integer(i64),
    String(String),
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

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::String(i) => write!(f, "{}", i),
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

    pub fn get(&self, index: usize) -> Result<&Value<'c>, ZeusError> {
        self.values.get(index).ok_or(ZeusError::ValueNotFound)
    }

    pub fn last(&self) -> Result<&Value<'c>, ZeusError> {
        self.values.last().ok_or(ZeusError::ValueNotFound)
    }

    pub fn last_mut(&mut self) -> Result<&mut Value<'c>, ZeusError> {
        self.values.last_mut().ok_or(ZeusError::ValueNotFound)
    }

    pub fn pop(&mut self) -> Result<Value, ZeusError> {
        self.values.pop().ok_or(ZeusError::ValueNotFound)
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.values.iter()
    }
}
