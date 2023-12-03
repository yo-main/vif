use std::collections::HashMap;

use crate::callframe::CallFrame;
use crate::error::InterpreterError;
use crate::error::RuntimeErrorType;
use crate::value_error;
use zeus_compiler::NativeFunctionCallee;
use zeus_compiler::OpCode;
use zeus_compiler::Variable;
use zeus_native::execute_native_call;
use zeus_values::value::Value;

pub struct VM<'function, 'stack, 'value, 'variables, 'globals>
where
    'globals: 'value,
{
    pub stack: &'stack mut Vec<Value<'value>>,
    pub variables: &'variables mut HashMap<String, Value<'value>>,
    pub globals: &'globals Vec<Variable>,
    pub frame: CallFrame<'stack, 'function>,
    pub previous_frames: Vec<CallFrame<'stack, 'function>>,
}

impl<'function, 'stack, 'value, 'variables, 'globals>
    VM<'function, 'stack, 'value, 'variables, 'globals>
where
    'globals: 'value,
    'value: 'function,
{
    pub fn new(
        stack: &'stack mut Vec<Value<'value>>,
        variables: &'variables mut HashMap<String, Value<'value>>,
        globals: &'globals Vec<Variable>,
        frame: CallFrame<'stack, 'function>,
    ) -> Self {
        VM {
            stack,
            variables,
            globals,
            frame,
            previous_frames: Vec::new(),
        }
    }

    pub fn interpret_loop(&mut self) -> Result<(), InterpreterError> {
        loop {
            match self.frame.ip.next() {
                None => {
                    if self.previous_frames.len() > 0 {
                        self.frame = self.previous_frames.pop().unwrap();
                    } else {
                        break;
                    }
                }
                Some(byte) => self.interpret(byte)?,
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, op_code: &OpCode) -> Result<(), InterpreterError> {
        log::debug!("op {}, stack: {:?}", op_code, self.stack);

        match op_code {
            OpCode::Print => {
                println!(
                    "printing {}",
                    self.stack.pop().ok_or(InterpreterError::Impossible)?
                );
            }
            OpCode::Pop => {
                self.stack.pop().ok_or(InterpreterError::Impossible)?;
            }
            OpCode::Return => {
                let result = self
                    .stack
                    .pop()
                    .ok_or(InterpreterError::CompileError(format!("Nothing to return")))
                    .unwrap();

                if self.previous_frames.len() == 0 {
                    self.stack.pop();
                    return Ok(());
                }

                self.stack.drain(self.frame.stack_position..);
                self.frame = self.previous_frames.pop().unwrap();
                self.stack.push(result);
            }
            OpCode::GlobalVariable(i) => {
                let var_name = match self.globals.get(*i) {
                    Some(Variable::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                self.variables.insert(
                    var_name.clone(),
                    self.stack.pop().ok_or(InterpreterError::Impossible)?,
                );
            }
            OpCode::Call(arg_count) => match &self.stack[self.stack.len() - arg_count - 1] {
                Value::Constant(Variable::Function(func)) => {
                    if func.arity != *arg_count {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::FunctionCall(format!(
                                "Expected {} parameters, got {}",
                                func.arity, arg_count
                            )),
                        ));
                    }

                    let mut new_frame = CallFrame {
                        function: func,
                        ip: func.chunk.iter(0),
                        stack_position: self.stack.len() - arg_count - 1,
                    };
                    std::mem::swap(&mut new_frame, &mut self.frame);
                    self.previous_frames.push(new_frame);
                }
                Value::Native(func) => {
                    if func.arity != *arg_count {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::FunctionCall(format!(
                                "Expected {} parameters, got {}",
                                func.arity, arg_count
                            )),
                        ));
                    }

                    let res = execute_native_call(self.stack, *arg_count, func).map_err(|e| {
                        InterpreterError::RuntimeError(RuntimeErrorType::FunctionFailed(format!(
                            "{e}"
                        )))
                    })?;

                    self.stack.push(res);
                }
                v => {
                    return Err(InterpreterError::CompileError(format!(
                        "Expected function, got {v}"
                    )))
                }
            },
            OpCode::Goto(i) => self.frame.reset_ip(*i),
            OpCode::Jump(i) => self.frame.ip.advance_by(*i).unwrap(),
            OpCode::JumpIfFalse(i) => {
                let value = self.stack.last().ok_or(InterpreterError::EmptyStack)?;

                match value {
                    Value::Boolean(false) => self.frame.ip.advance_by(*i).unwrap(),
                    Value::Integer(0) => self.frame.ip.advance_by(*i).unwrap(),
                    Value::Float(v) if v == &0.0 => self.frame.ip.advance_by(*i).unwrap(),
                    Value::Constant(Variable::Integer(0)) => self.frame.ip.advance_by(*i).unwrap(),
                    Value::Constant(Variable::Float(v)) if v == &0.0 => {
                        self.frame.ip.advance_by(*i).unwrap()
                    }
                    Value::String(s) if s.is_empty() => self.frame.ip.advance_by(*i).unwrap(),
                    Value::None => self.frame.ip.advance_by(*i).unwrap(),
                    _ => (),
                }
            }
            // WTF is that ?? It's working though but wow. I'll need to spend more time studying how
            OpCode::GetLocal(i) => self.stack.push(
                self.stack
                    .get(*i + self.frame.stack_position)
                    .unwrap()
                    .clone(),
            ),
            OpCode::SetLocal(i) => {
                self.stack[*i + self.frame.stack_position] = self.stack.last().unwrap().clone()
            }
            OpCode::GetGlobal(i) => match self.globals.get(*i) {
                Some(Variable::Identifier(s)) => match self.variables.get(s) {
                    Some(value) => self.stack.push(value.clone()),
                    _ => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::UndeclaredVariable(format!(
                                "Undeclared variable: {s}"
                            )),
                        ));
                    }
                },
                Some(Variable::Native(f)) => self.stack.push(Value::Native(f.clone())),
                _ => return Err(InterpreterError::Impossible),
            },
            OpCode::SetGlobal(i) => {
                let var_name = match self.globals.get(*i) {
                    Some(Variable::Identifier(s)) => s,
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
                        ));
                    }
                    _ => (),
                }
            }
            OpCode::Constant(i) => {
                let i = *i;
                match self.globals.get(i) {
                    Some(ref c) => self.stack.push(Value::Constant(c)),
                    _ => return Err(InterpreterError::ConstantNotFound),
                };
            }
            OpCode::Not => {
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
                                Variable::Function(f) => {
                                    return value_error!("Can't negate a function name: {f}")
                                }
                                Variable::Native(f) => {
                                    return value_error!("Can't negate a function name: {f}")
                                }
                            },
                            _ => return Err(InterpreterError::Impossible), // impossible
                        };

                        self.stack.push(new_value);
                    }
                    _ => return value_error!("Can't negate {value}"),
                };
            }
            OpCode::Negate => {
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
                                    return value_error!("Can't negate a variable name {f}")
                                }
                                Variable::String(_) => {
                                    return value_error!("Can't negate a string")
                                }
                                Variable::Function(f) => {
                                    return value_error!("Can't negate a function name: {f}")
                                }
                                Variable::Native(f) => {
                                    return value_error!("Can't negate a function name: {f}")
                                }
                            },
                            _ => return Err(InterpreterError::Impossible),
                        };

                        self.stack.push(new_value);
                    }
                    _ => return value_error!("Can't negate {value}"),
                };
            }
            OpCode::True => self.stack.push(Value::Boolean(true)),
            OpCode::False => self.stack.push(Value::Boolean(false)),
            OpCode::None => self.stack.push(Value::None),
            OpCode::Equal => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(a.eq(&b)))
            }
            OpCode::NotEqual => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(a.neq(&b)))
            }
            OpCode::Greater => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.gt(&a)?))
            }
            OpCode::GreaterOrEqual => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.gte(&a)?))
            }
            OpCode::Less => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.lt(&a)?))
            }
            OpCode::LessOrEqual => {
                let a = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                let b = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                self.stack.push(Value::Boolean(b.lte(&a)?))
            }
            OpCode::Add => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.add(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e.into()),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::Substract => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.substract(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e.into()),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::Multiply => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.multiply(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e.into()),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::Divide => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.divide(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => return Err(e.into()),
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
            OpCode::Modulo => {
                let other = self.stack.pop().ok_or(InterpreterError::EmptyStack)?;
                match self.stack.last_mut() {
                    Some(ptr) => match ptr.modulo(other) {
                        Ok(Some(value)) => {
                            self.stack.pop().ok_or(InterpreterError::Impossible)?;
                            self.stack.push(value);
                        }
                        Ok(None) => (),
                        Err(e) => {
                            return Err(InterpreterError::RuntimeError(
                                RuntimeErrorType::ValueError(format!("{e}")),
                            ))
                        }
                    },
                    None => return Err(InterpreterError::EmptyStack),
                }
            }
        };

        Ok(())
    }
}
