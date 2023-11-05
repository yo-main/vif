use crate::error::InterpreterError;
use crate::value::Value;
use zeus_compiler::Constant;
use zeus_compiler::OpCode;

pub struct VM {}

impl VM {
    pub fn new() -> Self {
        VM {}
    }

    pub fn interpret<'a, 'b>(
        &self,
        op_code: &OpCode,
        stack: &'a mut Vec<Value<'b>>,
        constants: &'b Vec<Constant>,
    ) -> Result<(), InterpreterError> {
        log::trace!("op {}, stack: {:?}", op_code, stack);
        match op_code {
            OpCode::OP_RETURN => {
                println!("{:?}", stack.pop().ok_or(InterpreterError::EmptyStack)?);
                return Ok(());
            }
            OpCode::OP_CONSTANT(i) => {
                let i = *i;
                match constants.get(i) {
                    Some(ref c) => stack.push(Value::Constant(c)),
                    None => return Err(InterpreterError::ConstantNotFound),
                };
            }
            OpCode::OP_NEGATE => {
                let value = stack.last_mut().ok_or(InterpreterError::EmptyStack)?;
                match value {
                    Value::Integer(ref mut i) => *i *= -1,
                    Value::Index(_) => {
                        return Err(InterpreterError::value_error(format!(
                            "Can't negate an index"
                        )))
                    }
                    Value::Constant(_) => {
                        let v = stack.pop().ok_or(InterpreterError::EmptyStack)?;
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

                        stack.push(new_value);
                    }
                    v => return Err(InterpreterError::value_error(format!("Can't negate {v}"))),
                };
            }
            OpCode::OP_ADD => {
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;

                match b {
                    Value::Integer(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j + i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't add int and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Index(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Index(j + i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't add Index and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Constant(Constant::Integer(i)) => {
                        let i = *i;
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j + i));
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
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;

                match b {
                    Value::Integer(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j -= i,
                            Value::Index(ref mut j) => *j -= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j - i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't substract int and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Index(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j -= i,
                            Value::Index(ref mut j) => *j -= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Index(j - i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't substract Index and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Constant(Constant::Integer(i)) => {
                        let i = *i;
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j + i));
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
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;

                match b {
                    Value::Integer(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j *= i,
                            Value::Index(ref mut j) => *j *= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j * i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't multiply int and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Index(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j *= i,
                            Value::Index(ref mut j) => *j *= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Index(j * i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't multiply Index and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Constant(Constant::Integer(i)) => {
                        let i = *i;
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j + i));
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
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;

                match b {
                    Value::Integer(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j /= i,
                            Value::Index(ref mut j) => *j /= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j / i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't divide int and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Index(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j /= i,
                            Value::Index(ref mut j) => *j /= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Index(j / i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't divide Index and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Constant(Constant::Integer(i)) => {
                        let i = *i;
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j + i));
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
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;

                match b {
                    Value::Integer(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j %= i,
                            Value::Index(ref mut j) => *j %= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j % i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't add modulo and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Index(i) => {
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j %= i,
                            Value::Index(ref mut j) => *j %= i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Index(j % i));
                            }
                            e => {
                                return Err(InterpreterError::value_error(format!(
                                    "Can't modulo Index and {}",
                                    e
                                )))
                            }
                        }
                    }
                    Value::Constant(Constant::Integer(i)) => {
                        let i = *i;
                        match stack.last_mut().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(ref mut j) => *j += i,
                            Value::Index(ref mut j) => *j += i,
                            Value::Constant(Constant::Integer(j)) => {
                                stack.pop().ok_or(InterpreterError::EmptyStack)?;
                                stack.push(Value::Integer(j + i));
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
        }

        Ok(())
    }
}
