use crate::callframe::CallFrame;
use crate::error::InterpreterError;
use crate::error::RuntimeErrorType;
use crate::value_error;
use zeus_compiler::Function;
use zeus_compiler::NativeFunction;
use zeus_compiler::Variable;
use zeus_native::execute_native_call;
use zeus_objects::global::Global;
use zeus_objects::op_code::OpCode;
use zeus_objects::stack::Stack;
use zeus_objects::value::Value;
use zeus_objects::variable_storage::VariableStore;

pub struct VM<'function, 'stack, 'value, 'variables, 'globals>
where
    'globals: 'value,
{
    pub stack: &'stack mut Stack<'value>,
    pub variables: &'variables mut VariableStore<'globals, 'value>,
    pub globals: &'globals Global,
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
        globals: &'globals Global,
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

    fn pop(&mut self) {
        self.stack.drop_last();
    }

    fn r#return(&mut self) {
        let result = self.stack.pop();

        if self.previous_frames.len() > 0 {
            self.stack.truncate(self.frame.get_position());
            self.frame = self.previous_frames.pop().unwrap();
            self.stack.push(result);
        }
    }

    fn global_variable(&mut self, i: usize) -> Result<(), InterpreterError> {
        if let Variable::Identifier(var_name) = self.globals.get(i) {
            self.variables.insert(var_name, self.stack.pop());
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
            Value::Constant(Variable::Function(func)) => self.call_function(func, arg_count),
            Value::Native(func) => self.call_native(func, arg_count),
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
            Value::Boolean(false) => self.frame.advance_by(i),
            Value::Integer(0) => self.frame.advance_by(i),
            Value::Float(v) if v == &0.0 => self.frame.advance_by(i),
            Value::Constant(Variable::Integer(0)) => self.frame.advance_by(i),
            Value::Constant(Variable::Float(v)) if v == &0.0 => self.frame.advance_by(i),
            Value::String(s) if s.is_empty() => self.frame.advance_by(i),
            Value::None => self.frame.advance_by(i),
            _ => (),
        }
    }

    fn get_local(&mut self, i: usize) {
        self.stack
            .push(self.stack.peek(i + self.frame.get_position()).clone())
    }

    fn set_local(&mut self, i: usize) {
        self.stack.set(
            i + self.frame.get_position(),
            self.stack.peek_last().clone(),
        )
    }

    fn get_global(&mut self, i: usize) -> Result<(), InterpreterError> {
        match self.globals.get(i) {
            Variable::Identifier(s) => self.stack.push(self.variables.get(s.as_str()).clone()),
            Variable::Native(f) => self.stack.push(Value::Native(f)),
            _ => return Err(InterpreterError::Impossible),
        }

        Ok(())
    }

    fn set_global(&mut self, i: usize) -> Result<(), InterpreterError> {
        if let Variable::Identifier(var_name) = self.globals.get(i) {
            if !self.variables.insert(
                var_name,
                self.stack.peek_last().clone(), // here we clone because the assignement might be part of a larger expression
                                                // the value must stay on the stack
            ) {
                return Err(InterpreterError::RuntimeError(
                    RuntimeErrorType::UndeclaredVariable(format!(
                        "Can't assign to undeclared variable: {var_name}"
                    )),
                ));
            }
        } else {
            return Err(InterpreterError::Impossible);
        };

        Ok(())
    }

    fn constant(&mut self, i: usize) {
        self.stack.push(Value::Constant(self.globals.get(i)))
    }

    fn assert_true(&mut self) -> Result<(), InterpreterError> {
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

        Ok(())
    }

    fn not(&mut self) -> Result<(), InterpreterError> {
        let value = self.stack.peek_last_mut();
        match value {
            Value::Integer(ref mut i) => *value = Value::Boolean(i == &0),
            Value::Index(ref mut i) => *value = Value::Boolean(i == &0),
            Value::Float(ref mut f) => *value = Value::Boolean(f == &0.0),
            Value::Boolean(ref mut b) => *b = !*b,
            Value::None => *value = Value::Boolean(true),
            Value::Constant(Variable::Integer(i)) => *value = Value::Boolean(*i == 0),
            Value::Constant(Variable::Float(f)) => *value = Value::Boolean(*f == 0.0),
            Value::Constant(Variable::String(s)) => *value = Value::Boolean(s.is_empty()),
            _ => return value_error!("Can't compare {value}"),
        };

        Ok(())
    }

    fn negate(&mut self) -> Result<(), InterpreterError> {
        let value = self.stack.peek_last_mut();
        match value {
            Value::Integer(ref mut i) => *i *= -1,
            Value::Float(ref mut f) => *f *= -1.0,
            Value::Boolean(ref mut b) => *b = b == &false,
            Value::Constant(Variable::Integer(i)) => *value = Value::Integer(i * -1),
            Value::Constant(Variable::Float(f)) => *value = Value::Float(f * -1.0),
            _ => return value_error!("Can't negate {value}"),
        };

        Ok(())
    }

    fn r#true(&mut self) {
        self.stack.push(Value::Boolean(true))
    }
    fn r#false(&mut self) {
        self.stack.push(Value::Boolean(false))
    }

    fn r#none(&mut self) {
        self.stack.push(Value::None)
    }

    fn equal(&mut self) {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = Value::Boolean(a.eq(&b))
    }

    fn not_equal(&mut self) {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = Value::Boolean(a.neq(&b))
    }

    fn greater(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = Value::Boolean(b.gt(&a)?);
        Ok(())
    }
    fn greater_or_equal(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = Value::Boolean(b.gte(&a)?);
        Ok(())
    }
    fn less(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = Value::Boolean(b.lt(&a)?);
        Ok(())
    }
    fn less_or_equal(&mut self) -> Result<(), InterpreterError> {
        let a = self.stack.pop();
        let b = self.stack.peek_last_mut();
        *b = Value::Boolean(b.lte(&a)?);
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

    pub fn interpret(&mut self, op_code: &OpCode) -> Result<(), InterpreterError> {
        // log::debug!("op {}, stack: {:?}", op_code, self.stack);

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
            OpCode::GetGlobal(i) => self.get_global(*i)?,
            OpCode::SetGlobal(i) => self.set_global(*i)?,
            OpCode::Constant(i) => self.constant(*i),
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
}
