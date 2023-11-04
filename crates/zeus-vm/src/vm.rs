use std::slice::Chunks;

use crate::debug::disassemble_instruction;
use crate::error::ZeusError;
use crate::opcode::Chunk;
use crate::opcode::OpCode;
use crate::value::Constant;
use crate::value::Value;
use crate::value::Values;

pub enum InterpreterError {
    Ok,
    CompileError(String),
    RuntimeError(String),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK error"),
            Self::CompileError(e) => write!(f, "Compiling error: {e}"),
            Self::RuntimeError(e) => write!(f, "Interpreter error: {e}"),
        }
    }
}

impl std::fmt::Debug for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK error"),
            Self::CompileError(e) => write!(f, "Compiling error: {e}"),
            Self::RuntimeError(e) => write!(f, "Interpreter error: {e}"),
        }
    }
}

pub struct VM<'c> {
    stack: Values<'c>,
}

impl<'c> VM<'c> {
    pub fn new() -> Self {
        VM {
            stack: Values::new(),
        }
    }

    pub fn interpret<'a>(&mut self, chunk: &'a mut Chunk<'c>) -> Result<(), InterpreterError>
    where
        'a: 'c,
    {
        self.run(chunk)
        // TODO: could we have an iterator somehow instead of ip ?
    }

    pub fn push(&mut self, value: Value<'c>) {
        self.stack.add(value);
    }

    pub fn pop(&mut self) -> Result<Value, InterpreterError> {
        self.stack
            .pop()
            .map_err(|e| InterpreterError::RuntimeError(format!("Empty stack")))
    }

    fn get_last(&mut self) -> Result<&mut Value<'c>, InterpreterError> {
        self.stack
            .last_mut()
            .map_err(|e| InterpreterError::RuntimeError(format!("Empty stack")))
    }

    fn get(&self) -> Result<&Value<'c>, InterpreterError> {
        self.stack
            .last()
            .map_err(|e| InterpreterError::RuntimeError(format!("Empty stack")))
    }

    fn run<'a>(&mut self, chunk: &'a mut Chunk<'c>) -> Result<(), InterpreterError>
    where
        'a: 'c,
    {
        for byte in chunk.iter() {
            log::trace!("op {}, stack: {:?}", byte, self.stack);
            match byte {
                OpCode::OP_RETURN => {
                    println!("{:?}", self.pop()?);
                    break;
                }
                OpCode::OP_CONSTANT(i) => {
                    let constant = chunk.get_constant(*i).map_err(|_| {
                        InterpreterError::CompileError(format!("Constant not found"))
                    })?;
                    self.push(Value::Constant(constant));
                }
                OpCode::OP_NEGATE => {
                    let value = self.get_last()?;
                    match value {
                        Value::Integer(ref mut i) => *i *= -1,
                        Value::Index(_) => {
                            return Err(InterpreterError::RuntimeError(format!(
                                "Can't negate an index"
                            )))
                        }
                        Value::Constant(_) => {
                            let v = self.pop()?;
                            let new_value = match v {
                                Value::Constant(c) => match c {
                                    Constant::Integer(i) => Value::Integer(i * -1),
                                    Constant::String(_) => {
                                        return Err(InterpreterError::RuntimeError(format!(
                                            "Can't negate a string"
                                        )))
                                    }
                                },
                                _ => return Err(InterpreterError::Ok), // impossible
                            };

                            self.push(new_value);
                        }
                        v => {
                            return Err(InterpreterError::RuntimeError(format!("Can't negate {v}")))
                        }
                    };
                }
                OpCode::OP_ADD => {
                    let b = self.pop()?;

                    match b {
                        Value::Integer(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Integer(j + i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't add int and {}",
                                    e
                                )))
                            }
                        },
                        Value::Index(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Index(j + i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't add Index and {}",
                                    e
                                )))
                            }
                        },
                        Value::Constant(Constant::Integer(i)) => {
                            let i = *i;
                            match self.get_last()? {
                                Value::Integer(ref mut j) => *j += i,
                                Value::Index(ref mut j) => *j += i,
                                Value::Constant(Constant::Integer(j)) => {
                                    self.pop()?;
                                    self.push(Value::Integer(j + i));
                                }
                                e => {
                                    return Err(InterpreterError::RuntimeError(format!(
                                        "Can't add Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::RuntimeError(format!(
                                "Can't use {} in a addition",
                                e
                            )))
                        }
                    }
                }
                OpCode::OP_SUBSTRACT => {
                    let b = self.pop()?;

                    match b {
                        Value::Integer(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j -= i,
                            Value::Index(ref mut j) => *j -= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Integer(j - i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't substract int and {}",
                                    e
                                )))
                            }
                        },
                        Value::Index(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j -= i,
                            Value::Index(ref mut j) => *j -= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Index(j - i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't substract Index and {}",
                                    e
                                )))
                            }
                        },
                        Value::Constant(Constant::Integer(i)) => {
                            let i = *i;
                            match self.get_last()? {
                                Value::Integer(ref mut j) => *j += i,
                                Value::Index(ref mut j) => *j += i,
                                Value::Constant(Constant::Integer(j)) => {
                                    self.pop()?;
                                    self.push(Value::Integer(j + i));
                                }
                                e => {
                                    return Err(InterpreterError::RuntimeError(format!(
                                        "Can't add Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::RuntimeError(format!(
                                "Can't use {} in a substraction",
                                e
                            )))
                        }
                    }
                }
                OpCode::OP_MULTIPLY => {
                    let b = self.pop()?;

                    match b {
                        Value::Integer(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j *= i,
                            Value::Index(ref mut j) => *j *= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Integer(j * i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't multiply int and {}",
                                    e
                                )))
                            }
                        },
                        Value::Index(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j *= i,
                            Value::Index(ref mut j) => *j *= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Index(j * i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't multiply Index and {}",
                                    e
                                )))
                            }
                        },
                        Value::Constant(Constant::Integer(i)) => {
                            let i = *i;
                            match self.get_last()? {
                                Value::Integer(ref mut j) => *j += i,
                                Value::Index(ref mut j) => *j += i,
                                Value::Constant(Constant::Integer(j)) => {
                                    self.pop()?;
                                    self.push(Value::Integer(j + i));
                                }
                                e => {
                                    return Err(InterpreterError::RuntimeError(format!(
                                        "Can't multiply Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::RuntimeError(format!(
                                "Can't use {} in a multiplication",
                                e
                            )))
                        }
                    }
                }
                OpCode::OP_DIVIDE => {
                    let b = self.pop()?;

                    match b {
                        Value::Integer(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j /= i,
                            Value::Index(ref mut j) => *j /= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Integer(j / i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't divide int and {}",
                                    e
                                )))
                            }
                        },
                        Value::Index(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j /= i,
                            Value::Index(ref mut j) => *j /= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Index(j / i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't divide Index and {}",
                                    e
                                )))
                            }
                        },
                        Value::Constant(Constant::Integer(i)) => {
                            let i = *i;
                            match self.get_last()? {
                                Value::Integer(ref mut j) => *j += i,
                                Value::Index(ref mut j) => *j += i,
                                Value::Constant(Constant::Integer(j)) => {
                                    self.pop()?;
                                    self.push(Value::Integer(j + i));
                                }
                                e => {
                                    return Err(InterpreterError::RuntimeError(format!(
                                        "Can't divide Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::RuntimeError(format!(
                                "Can't use {} in a division",
                                e
                            )))
                        }
                    }
                }
                OpCode::OP_MODULO => {
                    let b = self.pop()?;

                    match b {
                        Value::Integer(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j %= i,
                            Value::Index(ref mut j) => *j %= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Integer(j % i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't add modulo and {}",
                                    e
                                )))
                            }
                        },
                        Value::Index(i) => match self.get_last()? {
                            Value::Integer(ref mut j) => *j %= i,
                            Value::Index(ref mut j) => *j %= i,
                            Value::Constant(Constant::Integer(j)) => {
                                self.pop()?;
                                self.push(Value::Index(j % i));
                            }
                            e => {
                                return Err(InterpreterError::RuntimeError(format!(
                                    "Can't modulo Index and {}",
                                    e
                                )))
                            }
                        },
                        Value::Constant(Constant::Integer(i)) => {
                            let i = *i;
                            match self.get_last()? {
                                Value::Integer(ref mut j) => *j += i,
                                Value::Index(ref mut j) => *j += i,
                                Value::Constant(Constant::Integer(j)) => {
                                    self.pop()?;
                                    self.push(Value::Integer(j + i));
                                }
                                e => {
                                    return Err(InterpreterError::RuntimeError(format!(
                                        "Can't modulo Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::RuntimeError(format!(
                                "Can't use {} in a modulo",
                                e
                            )))
                        }
                    }
                }
                OpCode::OP_TEST(_) => (),
            }
        }

        Ok(())
    }

    fn read_constant<'a>(
        &self,
        chunk: &'a Chunk<'c>,
        i: usize,
    ) -> Result<&'c Constant, InterpreterError>
    where
        'a: 'c,
    {
        chunk
            .get_constant(i)
            .map_err(|_| InterpreterError::CompileError(format!("Constant not found")))
    }
}
