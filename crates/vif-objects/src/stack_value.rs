use crate::global::Global;

use crate::divide_by_zero_error;
use crate::errors::ValueError;
use crate::function::NativeFunction;
use crate::value_error;

#[derive(Debug, Clone)]
pub enum StackValue<'c> {
    Integer(i64),
    Index(i64),
    Float(f64),
    String(Box<String>),
    Global(&'c Global),
    BinaryOp(BinaryOp),
    Boolean(bool),
    Native(&'c NativeFunction),
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
}

impl std::fmt::Display for StackValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Index(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::Global(c) => write!(f, "{}", *c),
            Self::BinaryOp(o) => write!(f, "{}", o),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{}", s),
            Self::Native(s) => write!(f, "{}", s),
            Self::None => write!(f, "None"),
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
    values: Vec<StackValue<'c>>,
}
impl<'c> Values<'c> {
    pub fn new() -> Self {
        Values { values: Vec::new() }
    }

    pub fn add(&mut self, value: StackValue<'c>) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn get(&self, index: usize) -> Option<&StackValue<'c>> {
        self.values.get(index)
    }

    pub fn last(&self) -> Option<&StackValue<'c>> {
        self.values.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut StackValue<'c>> {
        self.values.last_mut()
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.values.pop()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, StackValue> {
        self.values.iter()
    }
}

impl StackValue<'_> {
    pub fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => i == j,
                Self::Index(j) => i == j,
                Self::Float(j) => *i as f64 == *j,
                Self::Global(Global::Integer(j)) => i == j,
                Self::Global(Global::Float(j)) => *i as f64 == *j,
                Self::Boolean(true) => i == &1,
                Self::Boolean(false) => i == &0,
                _ => false,
            },
            Self::Index(i) => match other {
                Self::Integer(j) => i == j,
                Self::Index(j) => i == j,
                Self::Float(j) => *i as f64 == *j,
                Self::Global(Global::Integer(j)) => i == j,
                Self::Global(Global::Float(j)) => *i as f64 == *j,
                Self::Boolean(true) => i == &1,
                Self::Boolean(false) => i == &0,
                _ => false,
            },
            Self::Float(i) => match other {
                Self::Integer(j) => *i == *j as f64,
                Self::Index(j) => *i == *j as f64,
                Self::Float(j) => i == j,
                Self::Global(Global::Integer(j)) => *i == *j as f64,
                Self::Global(Global::Float(j)) => i == j,
                Self::Boolean(true) => i == &1.0,
                Self::Boolean(false) => i == &0.0,
                _ => false,
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => i == j,
                Self::Index(j) => i == j,
                Self::Float(j) => *i as f64 == *j,
                Self::Global(Global::Integer(j)) => i == j,
                Self::Global(Global::Float(j)) => *i as f64 == *j,
                Self::Boolean(true) => i == &1,
                Self::Boolean(false) => i == &0,
                _ => false,
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => *i == *j as f64,
                Self::Index(j) => *i == *j as f64,
                Self::Float(j) => *i as f64 == *j,
                Self::Global(Global::Integer(j)) => *i == *j as f64,
                Self::Global(Global::Float(j)) => *i as f64 == *j,
                Self::Boolean(true) => i == &1.0,
                Self::Boolean(false) => i == &0.0,
                _ => false,
            },
            Self::None => match other {
                Self::None => true,
                _ => false,
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => i == j,
                Self::Integer(1) => *i == true,
                Self::Integer(0) => *i == false,
                Self::Float(v) if v == &1.0 => *i == true,
                Self::Float(v) if v == &0.0 => *i == false,
                Self::Global(Global::Integer(1)) => *i == true,
                Self::Global(Global::Float(v)) if v == &1.0 => *i == true,
                Self::Global(Global::Integer(0)) => *i == false,
                Self::Global(Global::Float(v)) if v == &0.0 => *i == false,
                _ => false,
            },
            Self::String(s1) => match other {
                Self::String(s2) => s1 == s2,
                Self::Global(Global::String(s2)) => s1.as_str() == s2.as_str(),
                _ => false,
            },
            Self::Global(Global::String(s1)) => match other {
                Self::String(s2) => s1.as_str() == s2.as_str(),
                Self::Global(Global::String(s2)) => s1 == s2,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn neq(&self, other: &Self) -> bool {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => i != j,
                Self::Index(j) => i != j,
                Self::Float(j) => *i as f64 != *j,
                Self::Global(Global::Integer(j)) => i != j,
                Self::Global(Global::Float(j)) => *i as f64 != *j,
                Self::Boolean(true) => i != &1,
                Self::Boolean(false) => i != &0,
                _ => false,
            },
            Self::Index(i) => match other {
                Self::Integer(j) => i != j,
                Self::Index(j) => i != j,
                Self::Float(j) => *i as f64 != *j,
                Self::Global(Global::Integer(j)) => i != j,
                Self::Global(Global::Float(j)) => *i as f64 != *j,
                Self::Boolean(true) => i != &1,
                Self::Boolean(false) => i != &0,
                _ => false,
            },
            Self::Float(i) => match other {
                Self::Integer(j) => *i != *j as f64,
                Self::Index(j) => *i != *j as f64,
                Self::Float(j) => i != j,
                Self::Global(Global::Integer(j)) => *i != *j as f64,
                Self::Global(Global::Float(j)) => i != j,
                Self::Boolean(true) => i != &1.0,
                Self::Boolean(false) => i != &0.0,
                _ => false,
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => i != j,
                Self::Index(j) => i != j,
                Self::Float(j) => *i as f64 != *j,
                Self::Global(Global::Integer(j)) => i != j,
                Self::Global(Global::Float(j)) => *i as f64 != *j,
                Self::Boolean(true) => i != &1,
                Self::Boolean(false) => i != &0,
                _ => false,
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => *i != *j as f64,
                Self::Index(j) => *i != *j as f64,
                Self::Float(j) => *i as f64 != *j,
                Self::Global(Global::Integer(j)) => *i != *j as f64,
                Self::Global(Global::Float(j)) => *i as f64 != *j,
                Self::Boolean(true) => i != &1.0,
                Self::Boolean(false) => i != &0.0,
                _ => false,
            },
            Self::None => match other {
                Self::None => true,
                _ => false,
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => i != j,
                Self::Integer(1) => *i != true,
                Self::Integer(0) => *i != false,
                Self::Float(v) if v == &1.0 => *i != true,
                Self::Float(v) if v == &0.0 => *i != false,
                Self::Global(Global::Integer(1)) => *i != true,
                Self::Global(Global::Float(v)) if v == &0.0 => *i != true,
                Self::Global(Global::Integer(0)) => *i != false,
                Self::Global(Global::Float(v)) if v == &1.0 => *i != false,
                _ => false,
            },
            Self::String(s1) => match other {
                Self::String(s2) => s1 != s2,
                Self::Global(Global::String(s2)) => s1.as_str() != s2.as_str(),
                _ => false,
            },
            Self::Global(Global::String(s1)) => match other {
                Self::String(s2) => s1.as_str() != s2.as_str(),
                Self::Global(Global::String(s2)) => s1 != s2,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn lt(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i < j),
                Self::Index(j) => Ok(i < j),
                Self::Float(j) => Ok((*i as f64) < *j),
                Self::Global(Global::Integer(j)) => Ok(i < j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) < *j),
                Self::Boolean(true) => Ok(i < &1),
                Self::Boolean(false) => Ok(i < &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Index(i) => match other {
                Self::Integer(j) => Ok(i < j),
                Self::Index(j) => Ok(i < j),
                Self::Float(j) => Ok((*i as f64) < *j),
                Self::Global(Global::Integer(j)) => Ok(i < j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) < *j),
                Self::Boolean(true) => Ok(i < &1),
                Self::Boolean(false) => Ok(i < &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i < *j as f64),
                Self::Index(j) => Ok(*i < *j as f64),
                Self::Float(j) => Ok(i < j),
                Self::Global(Global::Integer(j)) => Ok(*i < *j as f64),
                Self::Global(Global::Float(j)) => Ok(i < j),
                Self::Boolean(true) => Ok(i < &1.0),
                Self::Boolean(false) => Ok(i < &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => Ok(i < j),
                Self::Index(j) => Ok(i < j),
                Self::Float(j) => Ok((*i as f64) < *j),
                Self::Global(Global::Integer(j)) => Ok(i < j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) < *j),
                Self::Boolean(true) => Ok(i < &1),
                Self::Boolean(false) => Ok(i < &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => Ok(*i < (*j as f64)),
                Self::Index(j) => Ok(*i < (*j as f64)),
                Self::Float(j) => Ok((*i as f64) < *j),
                Self::Global(Global::Integer(j)) => Ok(*i < (*j as f64)),
                Self::Global(Global::Float(j)) => Ok((*i as f64) < *j),
                Self::Boolean(true) => Ok(i < &1.0),
                Self::Boolean(false) => Ok(i < &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i < j),
                Self::Integer(j) => Ok(&1 < j),
                Self::Index(j) => Ok(&1 < j),
                Self::Float(j) => Ok(&1.0 < j),
                Self::Global(Global::Integer(j)) => Ok(&1 < j),
                Self::Global(Global::Float(j)) => Ok(&1.0 < j),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            _ => return value_error!("Can't compare {self} and {other}"),
        }
    }

    pub fn lte(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i <= j),
                Self::Index(j) => Ok(i <= j),
                Self::Float(j) => Ok((*i as f64) <= *j),
                Self::Global(Global::Integer(j)) => Ok(i <= j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) <= *j),
                Self::Boolean(true) => Ok(i <= &1),
                Self::Boolean(false) => Ok(i <= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Index(i) => match other {
                Self::Integer(j) => Ok(i <= j),
                Self::Index(j) => Ok(i <= j),
                Self::Float(j) => Ok((*i as f64) <= *j),
                Self::Global(Global::Integer(j)) => Ok(i <= j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) <= *j),
                Self::Boolean(true) => Ok(i <= &1),
                Self::Boolean(false) => Ok(i <= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i <= *j as f64),
                Self::Index(j) => Ok(*i <= *j as f64),
                Self::Float(j) => Ok(i <= j),
                Self::Global(Global::Integer(j)) => Ok(*i <= *j as f64),
                Self::Global(Global::Float(j)) => Ok(i <= j),
                Self::Boolean(true) => Ok(i <= &1.0),
                Self::Boolean(false) => Ok(i <= &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => Ok(i <= j),
                Self::Index(j) => Ok(i <= j),
                Self::Float(j) => Ok((*i as f64) <= *j),
                Self::Global(Global::Integer(j)) => Ok(i <= j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) <= *j),
                Self::Boolean(true) => Ok(i <= &1),
                Self::Boolean(false) => Ok(i <= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => Ok(*i <= (*j as f64)),
                Self::Index(j) => Ok(*i <= (*j as f64)),
                Self::Float(j) => Ok((*i as f64) <= *j),
                Self::Global(Global::Integer(j)) => Ok(*i <= (*j as f64)),
                Self::Global(Global::Float(j)) => Ok((*i as f64) <= *j),
                Self::Boolean(true) => Ok(i <= &1.0),
                Self::Boolean(false) => Ok(i <= &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i <= j),
                Self::Integer(j) => Ok(&1 <= j),
                Self::Index(j) => Ok(&1 <= j),
                Self::Float(j) => Ok(&1.0 <= j),
                Self::Global(Global::Integer(j)) => Ok(&1 <= j),
                Self::Global(Global::Float(j)) => Ok(&1.0 <= j),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            _ => return value_error!("Can't compare {self} and {other}"),
        }
    }

    pub fn gt(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i > j),
                Self::Index(j) => Ok(i > j),
                Self::Float(j) => Ok((*i as f64) > *j),
                Self::Global(Global::Integer(j)) => Ok(i > j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) > *j),
                Self::Boolean(true) => Ok(i > &1),
                Self::Boolean(false) => Ok(i > &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Index(i) => match other {
                Self::Integer(j) => Ok(i > j),
                Self::Index(j) => Ok(i > j),
                Self::Float(j) => Ok((*i as f64) > *j),
                Self::Global(Global::Integer(j)) => Ok(i > j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) > *j),
                Self::Boolean(true) => Ok(i > &1),
                Self::Boolean(false) => Ok(i > &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i > *j as f64),
                Self::Index(j) => Ok(*i > *j as f64),
                Self::Float(j) => Ok(i > j),
                Self::Global(Global::Integer(j)) => Ok(*i > *j as f64),
                Self::Global(Global::Float(j)) => Ok(i > j),
                Self::Boolean(true) => Ok(i > &1.0),
                Self::Boolean(false) => Ok(i > &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => Ok(i > j),
                Self::Index(j) => Ok(i > j),
                Self::Float(j) => Ok((*i as f64) > *j),
                Self::Global(Global::Integer(j)) => Ok(i > j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) > *j),
                Self::Boolean(true) => Ok(i > &1),
                Self::Boolean(false) => Ok(i > &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => Ok(*i > (*j as f64)),
                Self::Index(j) => Ok(*i > (*j as f64)),
                Self::Float(j) => Ok((*i as f64) > *j),
                Self::Global(Global::Integer(j)) => Ok(*i > (*j as f64)),
                Self::Global(Global::Float(j)) => Ok((*i as f64) > *j),
                Self::Boolean(true) => Ok(i > &1.0),
                Self::Boolean(false) => Ok(i > &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i > j),
                Self::Integer(j) => Ok(&1 > j),
                Self::Index(j) => Ok(&1 > j),
                Self::Float(j) => Ok(&1.0 > j),
                Self::Global(Global::Integer(j)) => Ok(&1 > j),
                Self::Global(Global::Float(j)) => Ok(&1.0 > j),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            _ => return value_error!("Can't compare {self} and {other}"),
        }
    }

    pub fn gte(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i >= j),
                Self::Index(j) => Ok(i >= j),
                Self::Float(j) => Ok((*i as f64) >= *j),
                Self::Global(Global::Integer(j)) => Ok(i >= j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) >= *j),
                Self::Boolean(true) => Ok(i >= &1),
                Self::Boolean(false) => Ok(i >= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Index(i) => match other {
                Self::Integer(j) => Ok(i >= j),
                Self::Index(j) => Ok(i >= j),
                Self::Float(j) => Ok((*i as f64) >= *j),
                Self::Global(Global::Integer(j)) => Ok(i >= j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) >= *j),
                Self::Boolean(true) => Ok(i >= &1),
                Self::Boolean(false) => Ok(i >= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i >= *j as f64),
                Self::Index(j) => Ok(*i >= *j as f64),
                Self::Float(j) => Ok(i >= j),
                Self::Global(Global::Integer(j)) => Ok(*i >= *j as f64),
                Self::Global(Global::Float(j)) => Ok(i >= j),
                Self::Boolean(true) => Ok(i >= &1.0),
                Self::Boolean(false) => Ok(i >= &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => Ok(i >= j),
                Self::Index(j) => Ok(i >= j),
                Self::Float(j) => Ok((*i as f64) >= *j),
                Self::Global(Global::Integer(j)) => Ok(i >= j),
                Self::Global(Global::Float(j)) => Ok((*i as f64) >= *j),
                Self::Boolean(true) => Ok(i >= &1),
                Self::Boolean(false) => Ok(i >= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => Ok(*i >= (*j as f64)),
                Self::Index(j) => Ok(*i >= (*j as f64)),
                Self::Float(j) => Ok((*i as f64) >= *j),
                Self::Global(Global::Integer(j)) => Ok(*i >= (*j as f64)),
                Self::Global(Global::Float(j)) => Ok((*i as f64) >= *j),
                Self::Boolean(true) => Ok(i >= &1.0),
                Self::Boolean(false) => Ok(i >= &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i >= j),
                Self::Integer(j) => Ok(&1 >= j),
                Self::Index(j) => Ok(&1 >= j),
                Self::Float(j) => Ok(&1.0 >= j),
                Self::Global(Global::Integer(j)) => Ok(&1 >= j),
                Self::Global(Global::Float(j)) => Ok(&1.0 >= j),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            _ => return value_error!("Can't compare {self} and {other}"),
        }
    }

    pub fn add<'a, 'b>(&'a mut self, other: Self) -> Result<(), ValueError>
    where
        'b: 'a,
    {
        match self {
            Self::Integer(ref mut i) => match other {
                Self::Integer(j) => *i += j,
                Self::Float(j) => *self = StackValue::Float(j + *i as f64),
                Self::Boolean(true) => *i += 1,
                Self::Boolean(false) => (),
                Self::Global(Global::Integer(j)) => *i += j,
                Self::Global(Global::Float(j)) => *self = StackValue::Float(j + *i as f64),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i += j as f64,
                Self::Float(j) => *i += j,
                Self::Boolean(true) => *i += 1.0,
                Self::Boolean(false) => (),
                Self::Global(Global::Integer(j)) => *i += *j as f64,
                Self::Global(Global::Float(j)) => *i += j,
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => *self = StackValue::Integer(i + j),
                Self::Float(j) => *self = StackValue::Float(*i as f64 + j),
                Self::Boolean(true) => *self = StackValue::Integer(i + 1),
                Self::Boolean(false) => (),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(i + j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 + j),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => *self = StackValue::Float(i + j as f64),
                Self::Float(j) => *self = StackValue::Float(i + j),
                Self::Boolean(true) => *self = StackValue::Float(i + 1.0),
                Self::Boolean(false) => (),
                Self::Global(Global::Integer(j)) => *self = StackValue::Float(i + *j as f64),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(i + j),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Global(Global::String(i)) => match other {
                Self::Global(Global::String(j)) => {
                    *self = StackValue::String(Box::new(format!("{i}{j}")))
                }
                Self::String(j) => *self = StackValue::String(Box::new(format!("{i}{j}"))),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 + j),
                Self::Float(j) => *self = StackValue::Float(1.0 + j),
                Self::Boolean(true) => *self = StackValue::Integer(2),
                Self::Boolean(false) => *self = StackValue::Integer(1),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(1 + j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(1.0 + j),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(j),
                Self::Float(j) => *self = StackValue::Float(j),
                Self::Boolean(true) => *self = StackValue::Integer(1),
                Self::Boolean(false) => *self = StackValue::Integer(0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(*j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*j),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::String(i) => match other {
                Self::Global(Global::String(j)) => {
                    *self = StackValue::String(Box::new(format!("{i}{j}")))
                }
                Self::String(j) => *self = StackValue::String(Box::new(format!("{i}{j}"))),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::BinaryOp(_) => return value_error!("Can't add {self} and {other}"),
            _ => return value_error!("Can't add {self} and {other}"),
        };

        Ok(())
    }

    pub fn multiply<'a, 'b>(&'a mut self, other: Self) -> Result<(), ValueError>
    where
        'b: 'a,
    {
        match self {
            Self::Integer(ref mut i) => match other {
                Self::Integer(j) => *i *= j,
                Self::Float(j) => *self = StackValue::Float(*i as f64 * j),
                Self::Boolean(true) => *i *= 1,
                Self::Boolean(false) => *i *= 0,
                Self::Global(Global::Integer(j)) => *i *= j,
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 * j),
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i *= j as f64,
                Self::Float(j) => *i *= j,
                Self::Boolean(true) => *i *= 1.0,
                Self::Boolean(false) => *i *= 0.0,
                Self::Global(Global::Integer(j)) => *i *= *j as f64,
                Self::Global(Global::Float(j)) => *i *= j,
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => *self = StackValue::Integer(i * j),
                Self::Float(j) => *self = StackValue::Float(*i as f64 * j),
                Self::Boolean(true) => *self = StackValue::Integer(i * 1),
                Self::Boolean(false) => *self = StackValue::Integer(i * 0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(i * j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 * j),
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => *self = StackValue::Float(i * j as f64),
                Self::Float(j) => *self = StackValue::Float(i * j),
                Self::Boolean(true) => *self = StackValue::Float(i * 1.0),
                Self::Boolean(false) => *self = StackValue::Float(i * 0.0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Float(i * *j as f64),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(i * j),
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 * j),
                Self::Float(j) => *self = StackValue::Float(1.0 * j),
                Self::Boolean(true) => *self = StackValue::Integer(1 * 1),
                Self::Boolean(false) => *self = StackValue::Integer(1 * 0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(1 * j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(1.0 * j),
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 * j),
                Self::Float(j) => *self = StackValue::Float(0.0 * j),
                Self::Boolean(true) => *self = StackValue::Integer(0 * 1),
                Self::Boolean(false) => *self = StackValue::Integer(0 * 0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(0 * *j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(0.0 * *j),
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            _ => return value_error!("Can't multiply {self} and {other}"),
        };

        Ok(())
    }

    pub fn substract<'a, 'b>(&'a mut self, other: Self) -> Result<(), ValueError>
    where
        'b: 'a,
    {
        match self {
            Self::Integer(ref mut i) => match other {
                Self::Integer(j) => *i -= j,
                Self::Float(j) => *self = StackValue::Float(*i as f64 - j),
                Self::Boolean(true) => *i -= 1,
                Self::Boolean(false) => *i -= 0,
                Self::Global(Global::Integer(j)) => *i -= j,
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 - j),
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i -= j as f64,
                Self::Float(j) => *i -= j,
                Self::Boolean(true) => *i -= 1.0,
                Self::Boolean(false) => *i -= 0.0,
                Self::Global(Global::Integer(j)) => *i -= *j as f64,
                Self::Global(Global::Float(j)) => *i -= j,
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => *self = StackValue::Integer(i - j),
                Self::Float(j) => *self = StackValue::Float(*i as f64 - j),
                Self::Boolean(true) => *self = StackValue::Integer(i - 1),
                Self::Boolean(false) => *self = StackValue::Integer(i - 0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(i - j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 - j),
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => *self = StackValue::Float(i - j as f64),
                Self::Float(j) => *self = StackValue::Float(i - j),
                Self::Boolean(true) => *self = StackValue::Float(i - 1.0),
                Self::Boolean(false) => *self = StackValue::Float(i - 0.0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Float(i - *j as f64),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(i - j),
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 - j),
                Self::Float(j) => *self = StackValue::Float(1.0 - j),
                Self::Boolean(true) => *self = StackValue::Integer(1 - 1),
                Self::Boolean(false) => *self = StackValue::Integer(1 - 0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(1 - j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(1.0 - j),
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 - j),
                Self::Float(j) => *self = StackValue::Float(0.0 - j),
                Self::Boolean(true) => *self = StackValue::Integer(0 - 1),
                Self::Boolean(false) => *self = StackValue::Integer(0 - 0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(0 - *j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(0.0 - *j),
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            _ => return value_error!("Can't substract {self} and {other}"),
        };

        Ok(())
    }

    pub fn divide<'a, 'b>(&'a mut self, other: Self) -> Result<(), ValueError>
    where
        'b: 'a,
    {
        match self {
            Self::Integer(ref mut i) => match other {
                Self::Integer(j) => *i /= j,
                Self::Float(j) => *self = StackValue::Float(*i as f64 / j),
                Self::Boolean(true) => *i /= 1,
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide {i} by 0"),
                Self::Global(Global::Integer(j)) => *i /= j,
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 / j),
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i /= j as f64,
                Self::Float(j) => *i /= j,
                Self::Boolean(true) => *i /= 1.0,
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide {i} by 0"),
                Self::Global(Global::Integer(j)) => *i /= *j as f64,
                Self::Global(Global::Float(j)) => *i /= j,
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => *self = StackValue::Integer(i / j),
                Self::Float(j) => *self = StackValue::Float(*i as f64 / j),
                Self::Boolean(true) => *self = StackValue::Integer(i / 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide {i} by 0"),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(i / j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 / j),
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => *self = StackValue::Float(i / j as f64),
                Self::Float(j) => *self = StackValue::Float(i / j),
                Self::Boolean(true) => *self = StackValue::Float(i / 1.0),
                Self::Boolean(false) => *self = StackValue::Float(i / 0.0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Float(i / *j as f64),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(i / j),
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 / j),
                Self::Float(j) => *self = StackValue::Float(1.0 / j),
                Self::Boolean(true) => *self = StackValue::Integer(1 / 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 1 by Zero"),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(1 / j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(1.0 / j),
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 / j),
                Self::Float(j) => *self = StackValue::Float(0.0 / j),
                Self::Boolean(true) => *self = StackValue::Integer(0 / 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 0 by 0"),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(0 / *j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(0.0 / *j),
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            _ => return value_error!("Can't divide {self} and {other}"),
        };

        Ok(())
    }

    pub fn modulo<'a, 'b>(&'a mut self, other: Self) -> Result<(), ValueError>
    where
        'b: 'a,
    {
        match self {
            Self::Integer(ref mut i) => match other {
                Self::Integer(j) => *i %= j,
                Self::Float(j) => *self = StackValue::Float(*i as f64 % j),
                Self::Boolean(true) => *i %= 1,
                Self::Boolean(false) => return divide_by_zero_error!("Can't modulo {i} by 0"),
                Self::Global(Global::Integer(j)) => *i %= j,
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 % j),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i %= j as f64,
                Self::Float(j) => *i %= j,
                Self::Boolean(true) => *i %= 1.0,
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide {i} by 0"),
                Self::Global(Global::Integer(j)) => *i %= *j as f64,
                Self::Global(Global::Float(j)) => *i %= j,
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Global(Global::Integer(i)) => match other {
                Self::Integer(j) => *self = StackValue::Integer(i % j),
                Self::Float(j) => *self = StackValue::Float(*i as f64 % j),
                Self::Boolean(true) => *self = StackValue::Integer(i % 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide {i} by 0"),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(i % j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(*i as f64 % j),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Global(Global::Float(i)) => match other {
                Self::Integer(j) => *self = StackValue::Float(i % j as f64),
                Self::Float(j) => *self = StackValue::Float(i % j),
                Self::Boolean(true) => *self = StackValue::Float(i % 1.0),
                Self::Boolean(false) => *self = StackValue::Float(i % 0.0),
                Self::Global(Global::Integer(j)) => *self = StackValue::Float(i % *j as f64),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(i % j),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 % j),
                Self::Float(j) => *self = StackValue::Float(1.0 % j),
                Self::Boolean(true) => *self = StackValue::Integer(1 % 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 1 by Zero"),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(1 % j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(1.0 % j),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 % j),
                Self::Float(j) => *self = StackValue::Float(0.0 % j),
                Self::Boolean(true) => *self = StackValue::Integer(0 % 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 0 by 0"),
                Self::Global(Global::Integer(j)) => *self = StackValue::Integer(0 % *j),
                Self::Global(Global::Float(j)) => *self = StackValue::Float(0.0 % *j),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            _ => return value_error!("Can't modulo {self} and {other}"),
        };

        Ok(())
    }
}
