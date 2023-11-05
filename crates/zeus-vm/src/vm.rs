use crate::error::InterpreterError;
use crate::error::RuntimeErrorType;
use crate::opcode::Chunk;
use crate::opcode::OpCode;
use crate::value::Constant;
use crate::value::Value;
use crate::value::Values;

pub struct VM<'c> {
    stack: Vec<Value<'c>>,
}

impl<'c> VM<'c> {
    pub fn new() -> Self {
        VM { stack: Vec::new() }
    }

    pub fn interpret(&mut self, content: String) -> Result<(), InterpreterError> {
        let mut chunk = Chunk::new();
        chunk.compile(content)?;
        // let refe: &'c mut Chunk = &mut chunk;

        self.run(&mut chunk)?;

        return Ok(());
        // self.run(chunk)
        // TODO: could we have an iterator somehow instead of ip ?
    }

    pub fn push(&mut self, value: Value<'c>) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, InterpreterError> {
        self.stack.pop().ok_or(InterpreterError::EmptyStack)
    }

    fn get_last(&mut self) -> Result<&mut Value<'c>, InterpreterError> {
        self.stack.last_mut().ok_or(InterpreterError::EmptyStack)
    }

    fn get(&self) -> Result<&Value<'c>, InterpreterError> {
        self.stack.last().ok_or(InterpreterError::EmptyStack)
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
                    let i = *i;
                    let constant = chunk.get_constant(i)?;
                    self.push(Value::Constant(constant));
                }
                OpCode::OP_NEGATE => {
                    let value = self.get_last()?;
                    match value {
                        Value::Integer(ref mut i) => *i *= -1,
                        Value::Index(_) => {
                            return Err(InterpreterError::value_error(format!(
                                "Can't negate an index"
                            )))
                        }
                        Value::Constant(_) => {
                            let v = self.pop()?;
                            let new_value = match v {
                                Value::Constant(c) => match c {
                                    Constant::Integer(i) => Value::Integer(i * -1),
                                    Constant::String(_) => {
                                        return Err(InterpreterError::value_error(format!(
                                            "Can't negate a string"
                                        )))
                                    }
                                },
                                _ => return Err(InterpreterError::Ok), // impossible
                            };

                            self.push(new_value);
                        }
                        v => {
                            return Err(InterpreterError::value_error(format!("Can't negate {v}")))
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
                                return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                    return Err(InterpreterError::value_error(format!(
                                        "Can't add Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                    return Err(InterpreterError::value_error(format!(
                                        "Can't add Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                    return Err(InterpreterError::value_error(format!(
                                        "Can't multiply Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                    return Err(InterpreterError::value_error(format!(
                                        "Can't divide Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                return Err(InterpreterError::value_error(format!(
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
                                    return Err(InterpreterError::value_error(format!(
                                        "Can't modulo Integer and {}",
                                        e
                                    )))
                                }
                            }
                        }
                        e => {
                            return Err(InterpreterError::value_error(format!(
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
