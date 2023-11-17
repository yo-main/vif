use std::collections::HashMap;

use crate::error::InterpreterError;
use crate::error::RuntimeErrorType;
use crate::value::Value;
use crate::value_error;
use zeus_compiler::Constant;
use zeus_compiler::OpCode;

pub struct VM {}

impl VM {
    pub fn new() -> Self {
        VM {}
    }

    pub fn interpret<'a, 'b, 'c>(
        &self,
        op_code: &OpCode,
        stack: &'a mut Vec<Value<'b>>,
        variables: &'c mut HashMap<String, Value<'b>>,
        constants: &'b Vec<Constant>,
    ) -> Result<(), InterpreterError> {
        log::trace!("op {}, stack: {:?}", op_code, stack);
        match op_code {
            OpCode::OP_PRINT => {
                println!(
                    "printing {}",
                    stack.pop().ok_or(InterpreterError::Impossible)?
                );
            }
            OpCode::OP_POP => {
                stack.pop().ok_or(InterpreterError::Impossible)?;
            }
            OpCode::OP_RETURN => {}
            OpCode::OP_GLOBAL_VARIABLE(i) => {
                let var_name = match constants.get(*i) {
                    Some(Constant::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                variables.insert(
                    var_name.clone(),
                    stack.pop().ok_or(InterpreterError::Impossible)?,
                );
            }
            OpCode::OP_GET_GLOBAL(i) => {
                let var_name = match constants.get(*i) {
                    Some(Constant::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                match variables.get(var_name) {
                    Some(value) => stack.push(value.clone()),
                    _ => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::UndeclaredVariable(format!(
                                "Undeclared variable: {var_name}"
                            )),
                        ))
                    }
                }
            }
            OpCode::OP_SET_GLOBAL(i) => {
                let var_name = match constants.get(*i) {
                    Some(Constant::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                match variables.insert(
                    var_name.clone(),
                    stack.last().cloned().ok_or(InterpreterError::Impossible)?,
                    // here we clone because the assignement might be part of a larger expression
                    // the value must stay on the stack
                ) {
                    None => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::UndeclaredVariable(format!(
                                "Can't assign to undeclared variable: {var_name}"
                            )),
                        ))
                    }
                    _ => (),
                }
            }
            OpCode::OP_CONSTANT(i) => {
                let i = *i;
                match constants.get(i) {
                    Some(ref c) => stack.push(Value::Constant(c)),
                    None => return Err(InterpreterError::ConstantNotFound),
                };
            }
            OpCode::OP_NOT => {
                let value = stack.last_mut().ok_or(InterpreterError::EmptyStack)?;
                match value {
                    Value::Integer(_) => match stack.pop().ok_or(InterpreterError::EmptyStack)? {
                        Value::Integer(i) => stack.push(Value::Boolean(i == 0)),
                        _ => return Err(InterpreterError::Impossible),
                    },
                    Value::Index(_) => match stack.pop().ok_or(InterpreterError::EmptyStack)? {
                        Value::Index(i) => stack.push(Value::Boolean(i == 0)),
                        _ => return Err(InterpreterError::Impossible),
                    },
                    Value::Float(_) => match stack.pop().ok_or(InterpreterError::EmptyStack)? {
                        Value::Float(f) => stack.push(Value::Boolean(f == 0.0)),
                        _ => return Err(InterpreterError::Impossible),
                    },
                    Value::Boolean(ref mut b) => *b = !*b,
                    Value::None => *value = Value::Boolean(true),
                    Value::Constant(_) => {
                        let v = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                        let new_value = match v {
                            Value::Constant(c) => match c {
                                Constant::Integer(i) => Value::Boolean(*i == 0),
                                Constant::Float(f) => Value::Boolean(*f == 0.0),
                                Constant::String(s) => Value::Boolean(s.is_empty()),
                                Constant::Identifier(f) => {
                                    return value_error!("Can't negate a variable name")
                                }
                            },
                            _ => return Err(InterpreterError::Impossible), // impossible
                        };

                        stack.push(new_value);
                    }
                    Value::String(_) => return value_error!("Can't negate {value}"),
                    Value::BinaryOp(_) => return value_error!("Can't negate {value}"),
                };
            }
            OpCode::OP_NEGATE => {
                let value = stack.last_mut().ok_or(InterpreterError::EmptyStack)?;
                match value {
                    Value::Integer(ref mut i) => *i *= -1,
                    Value::Float(ref mut f) => *f *= -1.0,
                    Value::Index(_) => return value_error!("Can't negate index {value}"),
                    Value::Boolean(ref mut b) => *b = b == &false,
                    Value::Constant(_) => {
                        let v = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                        let new_value = match v {
                            Value::Constant(c) => match c {
                                Constant::Integer(i) => Value::Integer(i * -1),
                                Constant::Float(f) => Value::Float(f * -1.0),
                                Constant::Identifier(f) => {
                                    return value_error!("Can't negate a variable name")
                                }
                                Constant::String(_) => {
                                    return value_error!("Can't negate a string")
                                }
                            },
                            _ => return Err(InterpreterError::Impossible),
                        };

                        stack.push(new_value);
                    }
                    Value::String(_) => return value_error!("Can't negate {value}"),
                    Value::BinaryOp(_) => return value_error!("Can't negate {value}"),
                    Value::None => return value_error!("Can't negate {value}"),
                };
            }
            OpCode::OP_TRUE => stack.push(Value::Boolean(true)),
            OpCode::OP_FALSE => stack.push(Value::Boolean(false)),
            OpCode::OP_NONE => stack.push(Value::None),
            OpCode::OP_EQUAL => {
                let a = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                stack.push(Value::Boolean(a.eq(&b)))
            }
            OpCode::OP_NOT_EQUAL => {
                let a = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                stack.push(Value::Boolean(a.neq(&b)))
            }
            OpCode::OP_GREATER => {
                let a = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                stack.push(Value::Boolean(b.gt(&a)?))
            }
            OpCode::OP_GREATER_OR_EQUAL => {
                let a = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                stack.push(Value::Boolean(b.gte(&a)?))
            }
            OpCode::OP_LESS => {
                let a = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                stack.push(Value::Boolean(b.lt(&a)?))
            }
            OpCode::OP_LESS_OR_EQUAL => {
                let a = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                stack.push(Value::Boolean(b.lte(&a)?))
            }
            OpCode::OP_ADD => {
                let other = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match stack.last_mut() {
                    Some(ptr) => match ptr.add(other) {
                        Ok(Some(value)) => {
                            stack.pop().ok_or(InterpreterError::Impossible)?;
                            stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_SUBSTRACT => {
                let other = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match stack.last_mut() {
                    Some(ptr) => match ptr.substract(other) {
                        Ok(Some(value)) => {
                            stack.pop().ok_or(InterpreterError::Impossible)?;
                            stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_MULTIPLY => {
                let other = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match stack.last_mut() {
                    Some(ptr) => match ptr.multiply(other) {
                        Ok(Some(value)) => {
                            stack.pop().ok_or(InterpreterError::Impossible)?;
                            stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_DIVIDE => {
                let other = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match stack.last_mut() {
                    Some(ptr) => match ptr.divide(other) {
                        Ok(Some(value)) => {
                            stack.pop().ok_or(InterpreterError::Impossible)?;
                            stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_MODULO => {
                let other = stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match stack.last_mut() {
                    Some(ptr) => match ptr.modulo(other) {
                        Ok(Some(value)) => {
                            stack.pop().ok_or(InterpreterError::Impossible)?;
                            stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
        };

        Ok(())
    }
}
