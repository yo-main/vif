use crate::divide_by_zero_error;
use crate::errors::ValueError;
use crate::function::Function;
use crate::function::NativeFunction;
use crate::value_error;

#[derive(Clone)]
pub enum StackValue<'c> {
    StackReference(usize),
    Integer(i64),
    Float(f64),
    String(Box<String>),
    Boolean(bool),
    Function(&'c Function),
    Native(&'c NativeFunction),
    None,
}

impl std::fmt::Display for StackValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackReference(i) => write!(f, "&{}", i),
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(i) => write!(f, "{}", i),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{}", s),
            Self::Native(s) => write!(f, "&{}", s),
            Self::Function(s) => write!(f, "&{}", s),
            Self::None => write!(f, "None"),
        }
    }
}

impl std::fmt::Debug for StackValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackReference(i) => write!(f, "Ref[{}]", i),
            Self::Integer(i) => write!(f, "Int[{}]", i),
            Self::Float(i) => write!(f, "Float[{}]", i),
            Self::Boolean(b) => write!(f, "Bool[{}]", b),
            Self::String(s) => write!(f, "Str[{}]", s),
            Self::Native(s) => write!(f, "Nat[{}]", s),
            Self::Function(s) => write!(f, "Func[{}]", s),
            Self::None => write!(f, "None"),
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
                Self::Float(j) => *i as f64 == *j,
                Self::Boolean(true) => i == &1,
                Self::Boolean(false) => i == &0,
                _ => false,
            },
            Self::Float(i) => match other {
                Self::Integer(j) => *i == *j as f64,
                Self::Float(j) => i == j,
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
                _ => false,
            },
            Self::String(s1) => match other {
                Self::String(s2) => s1 == s2,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn neq(&self, other: &Self) -> bool {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => i != j,
                Self::Float(j) => *i as f64 != *j,
                Self::Boolean(true) => i != &1,
                Self::Boolean(false) => i != &0,
                _ => false,
            },
            Self::Float(i) => match other {
                Self::Integer(j) => *i != *j as f64,
                Self::Float(j) => i != j,
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
                _ => false,
            },
            Self::String(s1) => match other {
                Self::String(s2) => s1 != s2,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn lt(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i < j),
                Self::Float(j) => Ok((*i as f64) < *j),
                Self::Boolean(true) => Ok(i < &1),
                Self::Boolean(false) => Ok(i < &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i < *j as f64),
                Self::Float(j) => Ok(i < j),
                Self::Boolean(true) => Ok(i < &1.0),
                Self::Boolean(false) => Ok(i < &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i < j),
                Self::Integer(j) => Ok(&1 < j),
                Self::Float(j) => Ok(&1.0 < j),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            _ => return value_error!("Can't compare {self} and {other}"),
        }
    }

    pub fn lte(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i <= j),
                Self::Float(j) => Ok((*i as f64) <= *j),
                Self::Boolean(true) => Ok(i <= &1),
                Self::Boolean(false) => Ok(i <= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i <= *j as f64),
                Self::Float(j) => Ok(i <= j),
                Self::Boolean(true) => Ok(i <= &1.0),
                Self::Boolean(false) => Ok(i <= &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i <= j),
                Self::Integer(j) => Ok(&1 <= j),
                Self::Float(j) => Ok(&1.0 <= j),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            _ => return value_error!("Can't compare {self} and {other}"),
        }
    }

    pub fn gt(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i > j),
                Self::Float(j) => Ok((*i as f64) > *j),
                Self::Boolean(true) => Ok(i > &1),
                Self::Boolean(false) => Ok(i > &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i > *j as f64),
                Self::Float(j) => Ok(i > j),
                Self::Boolean(true) => Ok(i > &1.0),
                Self::Boolean(false) => Ok(i > &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i > j),
                Self::Integer(j) => Ok(&1 > j),
                Self::Float(j) => Ok(&1.0 > j),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            _ => return value_error!("Can't compare {self} and {other}"),
        }
    }

    pub fn gte(&self, other: &Self) -> Result<bool, ValueError> {
        match self {
            Self::Integer(i) => match other {
                Self::Integer(j) => Ok(i >= j),
                Self::Float(j) => Ok((*i as f64) >= *j),
                Self::Boolean(true) => Ok(i >= &1),
                Self::Boolean(false) => Ok(i >= &0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Float(i) => match other {
                Self::Integer(j) => Ok(*i >= *j as f64),
                Self::Float(j) => Ok(i >= j),
                Self::Boolean(true) => Ok(i >= &1.0),
                Self::Boolean(false) => Ok(i >= &0.0),
                _ => return value_error!("Can't compare {self} and {other}"),
            },
            Self::Boolean(i) => match other {
                Self::Boolean(j) => Ok(i >= j),
                Self::Integer(j) => Ok(&1 >= j),
                Self::Float(j) => Ok(&1.0 >= j),
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
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i += j as f64,
                Self::Float(j) => *i += j,
                Self::Boolean(true) => *i += 1.0,
                Self::Boolean(false) => (),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 + j),
                Self::Float(j) => *self = StackValue::Float(1.0 + j),
                Self::Boolean(true) => *self = StackValue::Integer(2),
                Self::Boolean(false) => *self = StackValue::Integer(1),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(j),
                Self::Float(j) => *self = StackValue::Float(j),
                Self::Boolean(true) => *self = StackValue::Integer(1),
                Self::Boolean(false) => *self = StackValue::Integer(0),
                _ => return value_error!("Can't add {self} and {other}"),
            },
            Self::String(i) => match other {
                Self::String(j) => *self = StackValue::String(Box::new(format!("{i}{j}"))),
                _ => return value_error!("Can't add {self} and {other}"),
            },
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
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i *= j as f64,
                Self::Float(j) => *i *= j,
                Self::Boolean(true) => *i *= 1.0,
                Self::Boolean(false) => *i *= 0.0,
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 * j),
                Self::Float(j) => *self = StackValue::Float(1.0 * j),
                Self::Boolean(true) => *self = StackValue::Integer(1 * 1),
                Self::Boolean(false) => *self = StackValue::Integer(1 * 0),
                _ => return value_error!("Can't multiply {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 * j),
                Self::Float(j) => *self = StackValue::Float(0.0 * j),
                Self::Boolean(true) => *self = StackValue::Integer(0 * 1),
                Self::Boolean(false) => *self = StackValue::Integer(0 * 0),
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
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i -= j as f64,
                Self::Float(j) => *i -= j,
                Self::Boolean(true) => *i -= 1.0,
                Self::Boolean(false) => *i -= 0.0,
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 - j),
                Self::Float(j) => *self = StackValue::Float(1.0 - j),
                Self::Boolean(true) => *self = StackValue::Integer(1 - 1),
                Self::Boolean(false) => *self = StackValue::Integer(1 - 0),
                _ => return value_error!("Can't substract {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 - j),
                Self::Float(j) => *self = StackValue::Float(0.0 - j),
                Self::Boolean(true) => *self = StackValue::Integer(0 - 1),
                Self::Boolean(false) => *self = StackValue::Integer(0 - 0),
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
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i /= j as f64,
                Self::Float(j) => *i /= j,
                Self::Boolean(true) => *i /= 1.0,
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide {i} by 0"),
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 / j),
                Self::Float(j) => *self = StackValue::Float(1.0 / j),
                Self::Boolean(true) => *self = StackValue::Integer(1 / 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 1 by Zero"),
                _ => return value_error!("Can't divide {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 / j),
                Self::Float(j) => *self = StackValue::Float(0.0 / j),
                Self::Boolean(true) => *self = StackValue::Integer(0 / 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 0 by 0"),
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
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Float(ref mut i) => match other {
                Self::Integer(j) => *i %= j as f64,
                Self::Float(j) => *i %= j,
                Self::Boolean(true) => *i %= 1.0,
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide {i} by 0"),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Boolean(true) => match other {
                Self::Integer(j) => *self = StackValue::Integer(1 % j),
                Self::Float(j) => *self = StackValue::Float(1.0 % j),
                Self::Boolean(true) => *self = StackValue::Integer(1 % 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 1 by Zero"),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            Self::Boolean(false) => match other {
                Self::Integer(j) => *self = StackValue::Integer(0 % j),
                Self::Float(j) => *self = StackValue::Float(0.0 % j),
                Self::Boolean(true) => *self = StackValue::Integer(0 % 1),
                Self::Boolean(false) => return divide_by_zero_error!("Can't divide 0 by 0"),
                _ => return value_error!("Can't modulo {self} and {other}"),
            },
            _ => return value_error!("Can't modulo {self} and {other}"),
        };

        Ok(())
    }
}
