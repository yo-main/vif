use std::collections::HashMap;

use crate::error::InterpreterError;
use crate::error::RuntimeErrorType;
use crate::value::Value;
use crate::value_error;
use zeus_compiler::Chunk;
use zeus_compiler::CompilerError;
use zeus_compiler::OpCode;
use zeus_compiler::Variable;

pub struct VM<'chunk, 'iter, 'stack, 'value, 'variables>
where
    'chunk: 'iter,
    'chunk: 'value,
{
    chunk: &'chunk Chunk,
    ip: std::slice::Iter<'iter, OpCode>,
    stack: &'stack mut Vec<Value<'value>>,
    variables: &'variables mut HashMap<String, Value<'value>>,
}

impl<'chunk, 'iter, 'stack, 'value, 'variables> VM<'chunk, 'iter, 'stack, 'value, 'variables>
where
    'chunk: 'iter,
{
    pub fn new(
        chunk: &'chunk Chunk,
        stack: &'stack mut Vec<Value<'value>>,
        variables: &'variables mut HashMap<String, Value<'value>>,
    ) -> Self {
        VM {
            chunk,
            stack,
            variables,
            ip: chunk.iter(0),
        }
    }

    pub fn interpret_loop(&mut self) -> Result<(), InterpreterError> {
        loop {
            match self.ip.next() {
                None => break,
                Some(byte) => self.interpret(byte)?,
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, op_code: &OpCode) -> Result<(), InterpreterError> {
        log::debug!("op {}, stack: {:?}", op_code, self.stack);
        match op_code {
            OpCode::OP_PRINT => {
                println!(
                    "printing {}",
                    self.stack.pop().ok_or(InterpreterError::Impossible)?
                );
            }
            OpCode::OP_POP => {
                self.stack.pop().ok_or(InterpreterError::Impossible)?;
            }
            OpCode::OP_RETURN => {}
            OpCode::OP_GLOBAL_VARIABLE(i) => {
                let var_name = match self.chunk.get_constant(*i) {
                    Ok(Variable::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                self.variables.insert(
                    var_name.clone(),
                    self.stack.pop().ok_or(InterpreterError::Impossible)?,
                );
            }
            OpCode::OP_LOOP(i) => self.ip = self.chunk.iter(*i),
            OpCode::OP_JUMP(i) => self.ip.advance_by(*i).unwrap(),
            OpCode::OP_JUMP_IF_FALSE(i) => {
                let value = self.stack.last().ok_or(InterpreterError::EmptyStack)?;

                match value {
                    Value::Boolean(false) => self.ip.advance_by(*i).unwrap(),
                    Value::Integer(0) => self.ip.advance_by(*i).unwrap(),
                    Value::Float(0.0) => self.ip.advance_by(*i).unwrap(),
                    Value::Constant(Variable::Integer(0)) => self.ip.advance_by(*i).unwrap(),
                    Value::Constant(Variable::Float(0.0)) => self.ip.advance_by(*i).unwrap(),
                    Value::String(s) if s.is_empty() => self.ip.advance_by(*i).unwrap(),
                    Value::None => self.ip.advance_by(*i).unwrap(),
                    _ => (),
                }
            }
            // WTF is that ?? It's working though but wow. I'll need to spend more time studying how
            OpCode::OP_GET_LOCAL(i) => self.stack.push(self.stack.get(*i).unwrap().clone()),
            OpCode::OP_SET_LOCAL(i) => self.stack[*i] = self.stack.last().unwrap().clone(),
            OpCode::OP_GET_GLOBAL(i) => {
                let var_name = match self.chunk.get_constant(*i) {
                    Ok(Variable::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                match self.variables.get(var_name) {
                    Some(value) => self.stack.push(value.clone()),
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
                let var_name = match self.chunk.get_constant(*i) {
                    Ok(Variable::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                match self.variables.insert(
                    var_name.clone(),
                    self.stack
                        .last()
                        .cloned()
                        .ok_or(InterpreterError::Impossible)?,
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
                match self.chunk.get_constant(i) {
                    Ok(ref c) => self.stack.push(Value::Constant(c)),
                    _ => return Err(InterpreterError::ConstantNotFound),
                };
            }
            OpCode::OP_NOT => {
                let value = self.stack.last_mut().ok_or(InterpreterError::EmptyStack)?;
                match value {
                    Value::Integer(_) => {
                        match self.stack.pop().ok_or(InterpreterError::EmptyStack)? {
                            Value::Integer(i) => self.stack.push(Value::Boolean(i == 0)),
                            _ => return Err(InterpreterError::Impossible),
                        }
                    }
                    Value::Index(_) => {
                        match self.stack.pop().ok_or(InterpreterError::EmptyStack)? {
                            Value::Index(i) => self.stack.push(Value::Boolean(i == 0)),
                            _ => return Err(InterpreterError::Impossible),
                        }
                    }
                    Value::Float(_) => {
                        match self.stack.pop().ok_or(InterpreterError::EmptyStack)? {
                            Value::Float(f) => self.stack.push(Value::Boolean(f == 0.0)),
                            _ => return Err(InterpreterError::Impossible),
                        }
                    }
                    Value::Boolean(ref mut b) => *b = !*b,
                    Value::None => *value = Value::Boolean(true),
                    Value::Constant(_) => {
                        let v = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                        let new_value = match v {
                            Value::Constant(c) => match c {
                                Variable::Integer(i) => Value::Boolean(*i == 0),
                                Variable::Float(f) => Value::Boolean(*f == 0.0),
                                Variable::String(s) => Value::Boolean(s.is_empty()),
                                Variable::Identifier(f) => {
                                    return value_error!("Can't negate a variable name: {f}")
                                }
                            },
                            _ => return Err(InterpreterError::Impossible), // impossible
                        };

                        self.stack.push(new_value);
                    }
                    Value::String(_) => return value_error!("Can't negate {value}"),
                    Value::BinaryOp(_) => return value_error!("Can't negate {value}"),
                };
            }
            OpCode::OP_NEGATE => {
                let value = self.stack.last_mut().ok_or(InterpreterError::EmptyStack)?;
                match value {
                    Value::Integer(ref mut i) => *i *= -1,
                    Value::Float(ref mut f) => *f *= -1.0,
                    Value::Index(_) => return value_error!("Can't negate index {value}"),
                    Value::Boolean(ref mut b) => *b = b == &false,
                    Value::Constant(_) => {
                        let v = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                        let new_value = match v {
                            Value::Constant(c) => match c {
                                Variable::Integer(i) => Value::Integer(i * -1),
                                Variable::Float(f) => Value::Float(f * -1.0),
                                Variable::Identifier(f) => {
                                    return value_error!("Can't negate a variable name")
                                }
                                Variable::String(_) => {
                                    return value_error!("Can't negate a string")
                                }
                            },
                            _ => return Err(InterpreterError::Impossible),
                        };

                        self.stack.push(new_value);
                    }
                    Value::String(_) => return value_error!("Can't negate {value}"),
                    Value::BinaryOp(_) => return value_error!("Can't negate {value}"),
                    Value::None => return value_error!("Can't negate {value}"),
                };
            }
            OpCode::OP_TRUE => self.stack.push(Value::Boolean(true)),
            OpCode::OP_FALSE => self.stack.push(Value::Boolean(false)),
            OpCode::OP_NONE => self.stack.push(Value::None),
            OpCode::OP_EQUAL => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(a.eq(&b)))
            }
            OpCode::OP_NOT_EQUAL => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(a.neq(&b)))
            }
            OpCode::OP_GREATER => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.gt(&a)?))
            }
            OpCode::OP_GREATER_OR_EQUAL => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.gte(&a)?))
            }
            OpCode::OP_LESS => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.lt(&a)?))
            }
            OpCode::OP_LESS_OR_EQUAL => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.lte(&a)?))
            }
            OpCode::OP_ADD => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.add(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_SUBSTRACT => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.substract(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_MULTIPLY => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.multiply(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_DIVIDE => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.divide(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::OP_MODULO => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.modulo(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
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
