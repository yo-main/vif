use std::net::Incoming;

use crate::callframe::CallFrame;
use crate::error::InterpreterError;
use crate::value_error;
use vif_native::execute_native_call;
use vif_objects::function::Function;
use vif_objects::function::NativeFunction;
use vif_objects::global::Global;
use vif_objects::global_store::GlobalStore;
use vif_objects::op_code::ItemReference;
use vif_objects::op_code::OpCode;
use vif_objects::stack::Stack;
use vif_objects::stack_value::StackValue;
use vif_objects::variable::InheritedLocalPos;
use vif_objects::variable_storage::VariableStore;

fn debug_stack(op_code: &OpCode, stack: &Stack, frame: &CallFrame) {
    println!(
        "{:>20} | {:<10} | {} | stack({}): {}",
        format!("{op_code}"),
        format!("{}", frame.get_function_name(),),
        frame.get_position(),
        stack.top,
        stack,
        //     self.globals,
        //     self.variables
    );
}

struct SoftTrunc {
    pos: usize,
    name: String,
}

impl std::fmt::Display for SoftTrunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ST {} {}", self.name, self.pos)
    }
}

pub struct VM<'content, 'function, 'stack, 'value, 'variables, 'globals>
where
    'globals: 'value,
{
    pub stack: &'stack mut Stack<'value>,
    pub variables: &'variables mut VariableStore<'globals, 'value>,
    pub globals: &'globals GlobalStore,
    pub frame: CallFrame<'stack, 'function>,
    pub previous_frames: Vec<CallFrame<'stack, 'function>>,
    soft_trunc: Option<SoftTrunc>,
    content: &'content str,
}

impl<'content, 'function, 'stack, 'value, 'variables, 'globals>
    VM<'content, 'function, 'stack, 'value, 'variables, 'globals>
where
    'globals: 'value,
    'value: 'function,
{
    pub fn new(
        stack: &'stack mut Stack<'value>,
        variables: &'variables mut VariableStore<'globals, 'value>,
        globals: &'globals GlobalStore,
        frame: CallFrame<'stack, 'function>,
        content: &'content str,
    ) -> Self {
        VM {
            stack,
            variables,
            globals,
            frame,
            previous_frames: Vec::with_capacity(100),
            soft_trunc: None,
            content,
        }
    }

    pub fn interpret_loop(&mut self) -> Result<(), InterpreterError> {
        loop {
            if let Some(byte) = self.frame.next() {
                self.interpret(byte)?;
            } else if !self.previous_frames.is_empty() {
                self.frame = self.previous_frames.pop().unwrap();
            } else {
                break;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn interpret(&mut self, op_code: &OpCode) -> Result<(), InterpreterError> {
        // debug_stack(op_code, self.stack, &self.frame);

        Ok(match op_code {
            OpCode::Global(i) => self.global(*i),
            OpCode::CreateLocal(i) => self.create_local(*i),
            OpCode::GetLocal(i) => self.get_local(*i),
            OpCode::GetInheritedLocal(v) => self.get_inherited_local(v),
            OpCode::GetGlobal(i) => self.get_global(*i),
            OpCode::SetLocal(i) => self.set_local(*i),
            OpCode::SetGlobal(i) => self.set_global(*i),
            OpCode::SetInheritedLocal(v) => self.set_inherited_local(v),
            OpCode::GlobalVariable(i) => self.global_variable(*i),
            OpCode::Call((arg_count, r)) => self.call(*arg_count).map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Add(r) => self.add().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Substract(r) => self.substract().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Pop => self.pop(),
            OpCode::Return(_) => self.r#return(),
            OpCode::Goto(i) => self.reset_ip(*i),
            OpCode::Jump(i) => self.advance_by(*i),
            OpCode::JumpIfFalse(i) => self.jump_if_false(*i),
            OpCode::AssertTrue(r) => self.assert_true().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Not(r) => self.not().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Negate(r) => self.negate().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::True(_) => self.r#true(),
            OpCode::False(_) => self.r#false(),
            OpCode::None(_) => self.r#none(),
            OpCode::Equal(_) => self.equal(),
            OpCode::NotEqual(_) => self.not_equal(),
            OpCode::Greater(r) => self.greater().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::GreaterOrEqual(r) => self.greater_or_equal().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Less(r) => self.less().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::LessOrEqual(r) => self.less_or_equal().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Multiply(r) => self.multiply().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Divide(r) => self.divide().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,

            OpCode::Modulo(r) => self.modulo().map_err(|mut e| {
                e.add_span(self.content, r);
                e
            })?,
            OpCode::NotImplemented => panic!("Not implemented"),
        })
    }

    #[inline]
    fn pop(&mut self) {
        self.stack.drop_last();
    }

    fn r#return(&mut self) {
        let result = self.stack.pop_till_scope(self.frame.get_position());
        // let result = self.stack.pop_raw();

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
                StackValue::Function(_) => {
                    self.soft_trunc = Some(SoftTrunc {
                        pos: self.frame.get_position(),
                        name: self.frame.get_function_name().to_owned(),
                    });
                }
                _ => self.stack.truncate(self.frame.get_position()),
            }

            self.frame = self.previous_frames.pop().unwrap();
            self.stack.push(result);
        }
    }

    #[inline]
    fn global_variable(&mut self, i: usize) {
        if let Global::Identifier(var_name) = self.globals.get(i) {
            self.variables.insert(
                var_name.name.as_str(),
                self.stack.pop_and_get_last().clone(),
            );
        } else {
            panic!("Impossible")
        }
    }

    #[inline]
    fn call_function(&mut self, func: &'function Function, arg_count: usize) {
        self.previous_frames.push(std::mem::replace(
            &mut self.frame,
            CallFrame::new(func, 0, self.stack.len() - arg_count - 1),
        ));

        if let Some(st) = &self.soft_trunc {
            if st.name == self.frame.get_function_name() {
                self.soft_trunc = None;
            }
        }
    }

    fn call_native(
        &mut self,
        func: &NativeFunction,
        arg_count: usize,
    ) -> Result<(), InterpreterError> {
        let res = execute_native_call(self.stack, arg_count, func)
            .map_err(|e| InterpreterError::FunctionFailed(format!("{e}")))?;
        self.stack.truncate(self.stack.len() - arg_count - 1);

        self.stack.push(res);
        Ok(())
    }

    #[inline]
    fn call(&mut self, arg_count: usize) -> Result<(), InterpreterError> {
        match self.stack.peek(self.stack.len() - arg_count - 1) {
            StackValue::Function(func) => self.call_function(func, arg_count),
            StackValue::Native(func) => self.call_native(func, arg_count)?,
            v => {
                return Err(InterpreterError::WrongValue(format!(
                    "Expected function, got {v}"
                )))
            }
        };

        Ok(())
    }

    #[inline]
    fn reset_ip(&mut self, i: usize) {
        self.frame.reset_ip(i)
    }

    #[inline]
    fn advance_by(&mut self, i: usize) {
        self.frame.advance_by(i).unwrap()
    }

    fn jump_if_false(&mut self, i: usize) {
        let value = self.stack.peek_last();

        match value {
            StackValue::Boolean(false) => self.frame.advance_by(i).unwrap(),
            StackValue::Integer(0) => self.frame.advance_by(i).unwrap(),
            StackValue::Float(v) if v == &0.0 => self.frame.advance_by(i).unwrap(),
            StackValue::String(s) if s.is_empty() => self.frame.advance_by(i).unwrap(),
            StackValue::None => self.frame.advance_by(i).unwrap(),
            _ => (),
        }
    }

    #[inline]
    fn get_local(&mut self, i: usize) {
        self.stack
            .push(self.stack.peek_first_ref(i + self.frame.get_position()));
    }

    fn create_local(&mut self, i: usize) {
        let value = self.stack.pop_raw();
        let index = i + self.frame.get_position();
        self.stack.set(index, value);

        if index == self.stack.top {
            self.stack.top += 1;
        }
    }

    fn set_local(&mut self, i: usize) {
        let value = self.stack.peek_last_raw().clone();
        self.stack.set(i + self.frame.get_position(), value);
    }

    #[inline]
    fn get_inherited_local(&mut self, pos: &InheritedLocalPos) {
        let index = self
            .previous_frames
            .iter()
            .nth(pos.get_depth())
            .map_or_else(
                || match &self.soft_trunc {
                    Some(st) => st.pos,
                    None => panic!("Bye"),
                },
                |f| f.get_position(),
            );

        self.stack
            .push(StackValue::StackReference(pos.get_pos() + index - 1))
    }

    fn set_inherited_local(&mut self, pos: &InheritedLocalPos) {
        let frame = self.previous_frames.iter().nth(pos.get_depth()).unwrap();
        let value = self.stack.peek_last_raw().clone();
        self.stack
            .set(pos.get_pos() + frame.get_position() - 1, value);
    }

    #[inline]
    fn get_global(&mut self, i: usize) {
        match self.globals.get(i) {
            Global::Identifier(s) => self.stack.push(self.variables.get(s.name.as_str()).clone()),
            Global::Native(f) => self.stack.push(StackValue::Native(f)),
            _ => panic!("Impossible"),
        }
    }

    fn set_global(&mut self, i: usize) {
        if let Global::Identifier(variable) = self.globals.get(i) {
            // we don't check if the result is false because the compiler should ensure we can't assign to a
            // variable that do not exist
            self.variables.insert(
                variable.name.as_str(),
                self.stack.peek_last().clone(), // here we clone because the assignement might be part of a larger expression
                                                // the value must stay on the stack
            );
        } else {
            panic!("Impossible");
        }
    }

    fn global(&mut self, i: usize) {
        self.stack.push(match self.globals.get(i) {
            Global::Integer(i) => StackValue::Integer(*i),
            Global::String(s) => StackValue::String(s.clone()),
            Global::Float(f) => StackValue::Float(*f),
            Global::Native(f) => StackValue::Native(f),
            Global::Function(f) => StackValue::Function(f),
            Global::Identifier(i) => panic!("Impossible - Got an identifier as value: {}", i),
        })
    }

    fn assert_true(&mut self) -> Result<(), InterpreterError> {
        let value = self.stack.peek_last();
        match value {
            StackValue::Integer(0) => {
                return Err(InterpreterError::AssertFail("0 is not true".to_owned()))
            }
            StackValue::Float(v) if v == &0.0 => {
                return Err(InterpreterError::AssertFail("0.0 is not true".to_owned()))
            }
            StackValue::String(s) if s.is_empty() => {
                return Err(InterpreterError::AssertFail(
                    "Empty string is not true".to_owned(),
                ))
            }
            StackValue::Boolean(false) => {
                return Err(InterpreterError::AssertFail("Assert failed".to_owned()))
            }
            StackValue::None => return Err(InterpreterError::AssertFail("is None".to_owned())),
            _ => (),
        }

        Ok(())
    }

    fn not(&mut self) -> Result<(), InterpreterError> {
        let value = self.stack.pop_and_get_last();
        let c = match value {
            StackValue::Integer(i) => StackValue::Boolean(i == &0),
            StackValue::Float(f) => StackValue::Boolean(f == &0.0),
            StackValue::Boolean(b) => StackValue::Boolean(!b),
            StackValue::None => StackValue::Boolean(true),
            _ => {
                return Err(InterpreterError::ValueError(
                    "Can't compare {value}".to_owned(),
                ))
            }
        };

        Ok(self.stack.push(c))
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
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.eq(a);
        self.stack.push(StackValue::Boolean(c));
    }

    fn not_equal(&mut self) {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.neq(a);
        self.stack.push(StackValue::Boolean(c));
    }

    fn greater(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.gt(a)?;
        Ok(self.stack.push(StackValue::Boolean(c)))
    }
    fn greater_or_equal(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.gte(a)?;
        Ok(self.stack.push(StackValue::Boolean(c)))
    }
    fn less(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.lt(a)?;
        Ok(self.stack.push(StackValue::Boolean(c)))
    }
    fn less_or_equal(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.lte(a)?;
        Ok(self.stack.push(StackValue::Boolean(c)))
    }
    fn add(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.add(a)?;
        Ok(self.stack.push(c))
    }
    fn substract(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.substract(a)?;
        Ok(self.stack.push(c))
    }
    fn multiply(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.multiply(a)?;
        Ok(self.stack.push(c))
    }
    fn divide(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.divide(a)?;
        Ok(self.stack.push(c))
    }
    fn modulo(&mut self) -> Result<(), InterpreterError> {
        let (a, b) = self.stack.pop_and_get_last_2_values();
        let c = b.modulo(a)?;
        Ok(self.stack.push(c))
    }
}
