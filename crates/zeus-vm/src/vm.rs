use std::collections::HashMap;

use crate::callframe::CallFrame;
use crate::error::InterpreterError;
use crate::error::RuntimeErrorType;
use crate::value_error;
use zeus_compiler::Variable;
use zeus_native::execute_native_call;
use zeus_objects::op_code::OpCode;
use zeus_objects::stack::Stack;
use zeus_objects::value::Value;

pub struct VM<'function, 'stack, 'value, 'variables, 'globals>
where
    'globals: 'value,
{
    pub stack: &'stack mut Stack<'value>,
    pub variables: &'variables mut HashMap<&'globals str, Value<'value>>,
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
        stack: &'stack mut Stack<'value>,
        variables: &'variables mut HashMap<&'globals str, Value<'value>>,
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
                println!("printing {}", self.stack.pop());
            }
            OpCode::Pop => {
                self.stack.drop_last();
            }
            OpCode::Return => {
                let result = self.stack.pop();

                if self.previous_frames.len() == 0 {
                    return Ok(());
                }

                self.stack.truncate(self.frame.stack_position);
                self.frame = self.previous_frames.pop().unwrap();
                self.stack.push(result);
            }
            OpCode::GlobalVariable(i) => {
                let var_name = match self.globals.get(*i) {
                    Some(Variable::Identifier(s)) => s,
                    _ => return Err(InterpreterError::Impossible),
                };

                self.variables.insert(var_name, self.stack.pop());
            }
            OpCode::Call(arg_count) => match self.stack.peek(self.stack.len() - arg_count - 1) {
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
                    self.stack.truncate(self.stack.len() - arg_count - 1);

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
                let value = self.stack.peek_last();

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
            OpCode::GetLocal(i) => self
                .stack
                .push(self.stack.peek(*i + self.frame.stack_position).clone()),
            OpCode::SetLocal(i) => self.stack.set(
                *i + self.frame.stack_position,
                self.stack.peek_last().clone(),
            ),
            OpCode::GetGlobal(i) => match self.globals.get(*i) {
                Some(Variable::Identifier(s)) => match self.variables.get(s.as_str()) {
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
                    var_name,
                    self.stack.peek_last().clone(), // here we clone because the assignement might be part of a larger expression
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
            OpCode::AssertTrue => {
                let value = self.stack.peek_last();
                match value {
                    Value::Integer(0) => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("0 is not true")),
                        ))
                    }
                    Value::Float(0.0) => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("0.0 is not true")),
                        ))
                    }
                    Value::String(s) if s.is_empty() => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("Empty string is not true")),
                        ))
                    }
                    Value::Boolean(false) => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("False")),
                        ))
                    }
                    Value::None => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("None")),
                        ))
                    }
                    Value::Constant(Variable::Integer(0)) => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("0 is not true")),
                        ))
                    }
                    Value::Constant(Variable::Float(0.0)) => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("0.0 is not true")),
                        ))
                    }
                    Value::Constant(Variable::String(s)) if s.is_empty() => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::AssertFail(format!("Empty string is not true")),
                        ))
                    }
                    _ => (),
                }
            }
            OpCode::Not => {
                let value = self.stack.peek_last_mut();
                match value {
                    Value::Integer(_) => match self.stack.pop() {
                        Value::Integer(i) => self.stack.push(Value::Boolean(i == 0)),
                        _ => return Err(InterpreterError::Impossible),
                    },
                    Value::Index(_) => match self.stack.pop() {
                        Value::Index(i) => self.stack.push(Value::Boolean(i == 0)),
                        _ => return Err(InterpreterError::Impossible),
                    },
                    Value::Float(_) => match self.stack.pop() {
                        Value::Float(f) => self.stack.push(Value::Boolean(f == 0.0)),
                        _ => return Err(InterpreterError::Impossible),
                    },
                    Value::Boolean(ref mut b) => *b = !*b,
                    Value::None => *value = Value::Boolean(true),
                    Value::Constant(_) => {
                        let v = self.stack.pop();
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
                let value = self.stack.peek_last_mut();
                match value {
                    Value::Integer(ref mut i) => *i *= -1,
                    Value::Float(ref mut f) => *f *= -1.0,
                    Value::Index(_) => return value_error!("Can't negate index {value}"),
                    Value::Boolean(ref mut b) => *b = b == &false,
                    Value::Constant(_) => {
                        let v = self.stack.pop();
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
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(Value::Boolean(a.eq(&b)))
            }
            OpCode::NotEqual => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(Value::Boolean(a.neq(&b)))
            }
            OpCode::Greater => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(Value::Boolean(b.gt(&a)?))
            }
            OpCode::GreaterOrEqual => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(Value::Boolean(b.gte(&a)?))
            }
            OpCode::Less => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(Value::Boolean(b.lt(&a)?))
            }
            OpCode::LessOrEqual => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(Value::Boolean(b.lte(&a)?))
            }
            OpCode::Add => {
                let other = self.stack.pop();
                let ptr = self.stack.peek_last_mut();
                match ptr.add(other) {
                    Ok(Some(value)) => {
                        self.stack.drop_last();
                        self.stack.push(value);
                    }
                    Ok(None) => (),
                    Err(e) => return Err(e.into()),
                }
            }
            OpCode::Substract => {
                let other = self.stack.pop();
                let ptr = self.stack.peek_last_mut();
                match ptr.substract(other) {
                    Ok(Some(value)) => {
                        self.stack.drop_last();
                        self.stack.push(value);
                    }
                    Ok(None) => (),
                    Err(e) => return Err(e.into()),
                }
            }
            OpCode::Multiply => {
                let other = self.stack.pop();
                let ptr = self.stack.peek_last_mut();
                match ptr.multiply(other) {
                    Ok(Some(value)) => {
                        self.stack.drop_last();
                        self.stack.push(value);
                    }
                    Ok(None) => (),
                    Err(e) => return Err(e.into()),
                }
            }
            OpCode::Divide => {
                let other = self.stack.pop();
                let ptr = self.stack.peek_last_mut();
                match ptr.divide(other) {
                    Ok(Some(value)) => {
                        self.stack.drop_last();
                        self.stack.push(value);
                    }
                    Ok(None) => (),
                    Err(e) => return Err(e.into()),
                }
            }
            OpCode::Modulo => {
                let other = self.stack.pop();
                let ptr = self.stack.peek_last_mut();
                match ptr.modulo(other) {
                    Ok(Some(value)) => {
                        self.stack.drop_last();
                        self.stack.push(value);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        return Err(InterpreterError::RuntimeError(
                            RuntimeErrorType::ValueError(format!("{e}")),
                        ))
                    }
                }
            }
        };

        Ok(())
    }
}
