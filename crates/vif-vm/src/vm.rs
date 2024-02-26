use std::fmt::Debug;

use crate::callframe::CallFrame;
use crate::error::InterpreterError;
use crate::error::RuntimeErrorType;
use crate::value_error;
use vif_native::execute_native_call;
use vif_objects::function::Function;
use vif_objects::function::NativeFunction;
use vif_objects::global::Global;
use vif_objects::global_store::GlobalStore;
use vif_objects::op_code::OpCode;
use vif_objects::stack::Stack;
use vif_objects::stack_value::StackValue;
use vif_objects::variable::InheritedLocalPos;
use vif_objects::variable_storage::VariableStore;

fn debug_stack(op_code: &OpCode, stack: &Stack, frame: &CallFrame) {
    println!(
        "{:>50} | {:<10} | stack({}): {}",
        format!("{op_code}"),
        format!("{}", frame.get_function_name(),),
        stack.top,
        stack,
        //     self.globals,
        //     self.variables
    );
}

pub struct VM<'function, 'stack, 'value, 'variables, 'globals>
where
    'globals: 'value,
{
    pub stack: &'stack mut Stack<'value>,
    pub variables: &'variables mut VariableStore<'globals, 'value>,
    pub globals: &'globals GlobalStore,
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
        variables: &'variables mut VariableStore<'globals, 'value>,
        globals: &'globals GlobalStore,
        frame: CallFrame<'stack, 'function>,
    ) -> Self {
        VM {
            stack,
            variables,
            globals,
            frame,
            previous_frames: Vec::with_capacity(100),
        }
    }

    pub fn interpret_loop(&mut self) -> Result<(), InterpreterError> {
        loop {
            if let Some(byte) = self.frame.next() {
                self.interpret(byte)?;
            } else if self.previous_frames.len() > 0 {
                self.frame = self.previous_frames.pop().unwrap();
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, op_code: &OpCode) -> Result<(), InterpreterError> {
        debug_stack(op_code, self.stack, &self.frame);

        match op_code {
            OpCode::Print => {
                println!("printing {}", self.stack.pop());
            }
            OpCode::Pop => self.pop(),
            OpCode::Return => self.r#return(),
            OpCode::GlobalVariable(i) => self.global_variable(*i)?,
            OpCode::Call(arg_count) => self.call(*arg_count)?,
            OpCode::Goto(i) => self.reset_ip(*i),
            OpCode::Jump(i) => self.advance_by(*i),
            OpCode::JumpIfFalse(i) => self.jump_if_false(*i),
            // WTF is that ?? It's working though but wow. I'll need to spend more time studying how
            OpCode::GetLocal(i) => self.get_local(*i),
            OpCode::SetLocal(i) => self.set_local(*i),
            OpCode::GetInheritedLocal(v) => self.get_inherited_local(v),
            OpCode::SetInheritedLocal(v) => self.set_inherited_local(v),
            OpCode::GetGlobal(i) => self.get_global(*i)?,
            OpCode::SetGlobal(i) => self.set_global(*i)?,
            OpCode::Global(i) => self.global(*i)?,
            OpCode::AssertTrue => self.assert_true()?,
            OpCode::Not => self.not()?,
            OpCode::Negate => self.negate()?,
            OpCode::True => self.r#true(),
            OpCode::False => self.r#false(),
            OpCode::None => self.r#none(),
            OpCode::Equal => self.equal(),
            OpCode::NotEqual => self.not_equal(),
            OpCode::Greater => self.greater()?,
            OpCode::GreaterOrEqual => self.greater_or_equal()?,
            OpCode::Less => self.less()?,
            OpCode::LessOrEqual => self.less_or_equal()?,
            OpCode::Add => self.add()?,
            OpCode::Substract => self.substract()?,
            OpCode::Multiply => self.multiply()?,
            OpCode::Divide => self.divide()?,
            OpCode::Modulo => self.modulo()?,
            OpCode::NotImplemented => panic!("Not implemented"),
        };

        Ok(())
    }

    fn pop(&mut self) {
        self.stack.drop_last();
    }

    fn r#return(&mut self) {
        let result = self.stack.pop();

        if self.previous_frames.len() > 0 {
            match result {
                // function hasn't been called yet, don't get rid of its variable
                // it's a hack that won't work all the time (especially after we add classes)
                // TODO: store inherited variables on the function itself, not the stack
                //
                // careful about
                //
                // def coucou():
                //     var x = 1
                //     x = 2
                //     def closure():
                //         x = 1
                //
                // when x=2, we need to know it's a inherited variable
                StackValue::Function(_) => (),
                _ => self.stack.truncate(self.frame.get_position()),
            }

            self.frame = self.previous_frames.pop().unwrap();
            self.stack.push(result);
        }
    }

    fn global_variable(&mut self, i: usize) -> Result<(), InterpreterError> {
        if let Global::Identifier(var_name) = self.globals.get(i) {
            self.variables
                .insert(var_name.name.as_str(), self.stack.pop());
        } else {
            return Err(InterpreterError::Impossible);
        }

        Ok(())
    }

    fn call_function(
        &mut self,
        func: &'function Function,
        arg_count: usize,
    ) -> Result<(), InterpreterError> {
        if func.arity != arg_count {
            return Err(InterpreterError::RuntimeError(
                RuntimeErrorType::FunctionCall(format!(
                    "Expected {} parameters, got {}",
                    func.arity, arg_count
                )),
            ));
        }

        let new_frame = self.frame.start_new(func, self.stack.len() - arg_count - 1);
        self.previous_frames
            .push(std::mem::replace(&mut self.frame, new_frame));
        Ok(())
    }

    fn call_native(
        &mut self,
        func: &NativeFunction,
        arg_count: usize,
    ) -> Result<(), InterpreterError> {
        if func.arity != arg_count {
            return Err(InterpreterError::RuntimeError(
                RuntimeErrorType::FunctionCall(format!(
                    "Expected {} parameters, got {}",
                    func.arity, arg_count
                )),
            ));
        }

        let res = execute_native_call(self.stack, arg_count, func).map_err(|e| {
            InterpreterError::RuntimeError(RuntimeErrorType::FunctionFailed(format!("{e}")))
        })?;
        self.stack.truncate(self.stack.len() - arg_count - 1);

        self.stack.push(res);
        Ok(())
    }

    fn call(&mut self, arg_count: usize) -> Result<(), InterpreterError> {
        match self.stack.peek(self.stack.len() - arg_count - 1) {
            StackValue::Function(func) => self.call_function(func, arg_count),
            StackValue::Native(func) => self.call_native(func, arg_count),
            v => {
                return Err(InterpreterError::CompileError(format!(
                    "Expected function, got {v}"
                )))
            }
        }
    }

    fn reset_ip(&mut self, i: usize) {
        self.frame.reset_ip(i)
    }

    fn advance_by(&mut self, i: usize) {
        self.frame.advance_by(i)
    }

    fn jump_if_false(&mut self, i: usize) {
        let value = self.stack.peek_last();

        match value {
            StackValue::Boolean(false) => self.frame.advance_by(i),
            StackValue::Integer(0) => self.frame.advance_by(i),
            StackValue::Float(v) if v == &0.0 => self.frame.advance_by(i),
            StackValue::String(s) if s.is_empty() => self.frame.advance_by(i),
            StackValue::None => self.frame.advance_by(i),
            _ => (),
        }
    }

    fn get_local(&mut self, i: usize) {
        self.stack
            .push(StackValue::LocalReference(i + self.frame.get_position()));
    }

    fn set_local(&mut self, i: usize) {
        let value = self.stack.peek_last_mut().clone();
        self.stack.set(i + self.frame.get_position(), value);
    }

    fn get_inherited_local(&mut self, pos: &InheritedLocalPos) {
        let frame = self.previous_frames.iter().nth(pos.depth - 1).unwrap();
        self.stack
            .push(StackValue::LocalReference(pos.pos + frame.get_position()))
    }

    fn set_inherited_local(&mut self, pos: &InheritedLocalPos) {
        let frame = self.previous_frames.iter().nth(pos.depth - 1).unwrap();
        let value = self.stack.peek_last_mut().clone();
        self.stack.set(pos.pos + frame.get_position(), value);
    }

    fn get_global(&mut self, i: usize) -> Result<(), InterpreterError> {
        match self.globals.get(i) {
            Global::Identifier(s) => match self.variables.get(s.name.as_str()) {
                None => {
                    return Err(InterpreterError::RuntimeError(
                        RuntimeErrorType::UndeclaredVariable(format!("{}", s)),
                    ))
                }
                Some(var) => self.stack.push(var.clone()),
            },
            Global::Native(f) => self.stack.push(StackValue::Native(f)),
            _ => return Err(InterpreterError::Impossible),
        }

        Ok(())
    }

    fn set_global(&mut self, i: usize) -> Result<(), InterpreterError> {
        if let Global::Identifier(variable) = self.globals.get(i) {
            if !self.variables.insert(
                variable.name.as_str(),
                self.stack.peek_last().clone(), // here we clone because the assignement might be part of a larger expression
                                                // the value must stay on the stack
            ) {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::UndeclaredVariable(format!(
                        "Can't assign to undeclared variable: {variable}"
                    )),
                ));
            }
        } else {
            return Err(InterpreterError::Impossible);
        };

        Ok(())
    }

    fn global(&mut self, i: usize) -> Result<(), InterpreterError> {
        self.stack.push(match self.globals.get(i) {
            Global::Integer(i) => StackValue::Integer(*i),
            Global::String(s) => StackValue::String(s.clone()),
            Global::Float(f) => StackValue::Float(*f),
            Global::Native(f) => StackValue::Native(f),
            Global::Function(f) => StackValue::Function(f),
            Global::Identifier(i) => {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::ValueError(format!("Got an identifier as value: {}", i)),
                ))
            }
        });

        Ok(())
    }

    fn assert_true(&mut self) -> Result<(), InterpreterError> {
        // debug_stack("assert", self.stack, &self.frame, &self.previous_frames);
        let value = self.stack.peek_last();
        match value {
            StackValue::Integer(0) => {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::AssertFail(format!("0 is not true")),
                ))
            }
            StackValue::Float(v) if v == &0.0 => {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::AssertFail(format!("0.0 is not true")),
                ))
            }
            StackValue::String(s) if s.is_empty() => {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::AssertFail(format!("Empty string is not true")),
                ))
            }
            StackValue::Boolean(false) => {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::AssertFail(format!("False")),
                ))
            }
            StackValue::None => {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::AssertFail(format!("None")),
                ))
            }
            _ => (),
        }

        Ok(())
    }

    fn not(&mut self) -> Result<(), InterpreterError> {
        let value = self.stack.peek_last_mut();
        match value {
            StackValue::Integer(ref mut i) => *value = StackValue::Boolean(i == &0),
            StackValue::Float(ref mut f) => *value = StackValue::Boolean(f == &0.0),
            StackValue::Boolean(ref mut b) => *b = !*b,
            StackValue::None => *value = StackValue::Boolean(true),
            _ => return value_error!("Can't compare {value}"),
        };

        Ok(())
    }

    fn negate(&mut self) -> Result<(), InterpreterError> {
        let value = self.stack.peek_last_mut();
        match value {
            StackValue::Integer(ref mut i) => *i *= -1,
            StackValue::Float(ref mut f) => *f *= -1.0,
            StackValue::Boolean(ref mut b) => *b = b == &false,
            _ => return value_error!("Can't negate {value}"),
        };

        Ok(())
    }

    fn r#true(&mut self) {
        self.stack.push(StackValue::Boolean(true))
    }
    fn r#false(&mut self) {
        self.stack.push(StackValue::Boolean(false))
    }

    fn r#none(&mut self) {
        self.stack.push(StackValue::None)
    }

    fn equal(&mut self) {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = StackValue::Boolean(a.eq(&b))
    }

    fn not_equal(&mut self) {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = StackValue::Boolean(a.neq(&b))
    }

    fn greater(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = StackValue::Boolean(b.gt(&a)?);
        Ok(())
    }
    fn greater_or_equal(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = StackValue::Boolean(b.gte(&a)?);
        Ok(())
    }
    fn less(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = StackValue::Boolean(b.lt(&a)?);
        Ok(())
    }
    fn less_or_equal(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = StackValue::Boolean(b.lte(&a)?);
        Ok(())
    }
    fn add(&mut self) -> Result<(), InterpreterError> {
        let other = self.stack.pop();
        self.stack.peek_last_mut().add(other)?;
        Ok(())
    }
    fn substract(&mut self) -> Result<(), InterpreterError> {
        let other = self.stack.pop();

        self.stack.peek_last_mut().substract(other)?;
        Ok(())
    }
    fn multiply(&mut self) -> Result<(), InterpreterError> {
        let other = self.stack.pop();
        self.stack.peek_last_mut().multiply(other)?;
        Ok(())
    }
    fn divide(&mut self) -> Result<(), InterpreterError> {
        let other = self.stack.pop();
        self.stack.peek_last_mut().divide(other)?;
        Ok(())
    }
    fn modulo(&mut self) -> Result<(), InterpreterError> {
        let other = self.stack.pop();
        self.stack.peek_last_mut().modulo(other)?;
        Ok(())
    }
}
