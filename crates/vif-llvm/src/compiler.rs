use crate::builder::Builder;
use crate::error::CompilerError;
use crate::Global;
use crate::NativeFunction;
use crate::NativeFunctionCallee;
use crate::OpCode;

use inkwell;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::types::BasicTypeEnum;
use inkwell::values::AsValueRef;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::values::BasicValue;
use inkwell::values::BasicValueEnum;
use inkwell::values::FunctionValue;
use inkwell::values::IntMathValue;
use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

use vif_loader::log;
use vif_objects::ast;
use vif_objects::function::Arity;
use vif_objects::function::Function;
use vif_objects::global_store::GlobalStore;
use vif_objects::op_code::ItemReference;
use vif_objects::span::Span;
use vif_objects::variable::InheritedLocalPos;
use vif_objects::variable::InheritedVariable;
use vif_objects::variable::Variable;
use vif_objects::variable::VariableType;

#[derive(Clone, Copy)]
pub enum LLVMValue<'ctx> {
    BasicValueEnum(BasicValueEnum<'ctx>),
    FunctionValue(FunctionValue<'ctx>),
}

impl<'ctx> LLVMValue<'ctx> {
    pub fn get_basic_value_enum(self) -> BasicValueEnum<'ctx> {
        match self {
            Self::BasicValueEnum(v) => v,
            _ => unreachable!(),
        }
    }

    pub fn get_function_value(self) -> FunctionValue<'ctx> {
        match self {
            Self::FunctionValue(f) => f,
            _ => unreachable!(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::BasicValueEnum(v) => v.get_name().to_str().unwrap().to_owned(),
            Self::FunctionValue(v) => v.get_name().to_str().unwrap().to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
struct StoredVariable<'ctx> {
    ptr: BasicValueEnum<'ctx>,
    // v: &'function ast::Variable,
}

impl<'ctx> StoredVariable<'ctx> {
    fn new(ptr: BasicValueEnum<'ctx>) -> Self {
        Self { ptr }
    }
}

#[derive(Debug, Clone)]
struct StoredFunction<'ctx> {
    ptr: FunctionValue<'ctx>,
    // f: &'function ast::Function,
}

impl<'ctx, 'function> StoredFunction<'ctx> {
    fn new(ptr: FunctionValue<'ctx>) -> Self {
        Self { ptr }
    }
}

#[derive(Debug, Clone)]
struct Variables<'ctx> {
    data: HashMap<String, StoredVariable<'ctx>>,
}

impl<'ctx> Variables<'ctx> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, var_name: String, value: BasicValueEnum<'ctx>) {
        self.data.insert(var_name, StoredVariable::new(value));
    }

    fn get(&self, var_name: String) -> Option<&StoredVariable<'ctx>> {
        self.data.get(&var_name)
    }
}

#[derive(Debug, Clone)]
struct Functions<'ctx> {
    data: HashMap<String, StoredFunction<'ctx>>,
}

impl<'ctx, 'function> Functions<'ctx> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, var_name: String, value: FunctionValue<'ctx>) {
        self.data.insert(var_name, StoredFunction::new(value));
    }

    fn get(&self, var_name: String) -> Option<&StoredFunction<'ctx>> {
        self.data.get(&var_name)
    }
}

#[derive(Debug, Clone)]
pub struct Store<'ctx> {
    variables: Variables<'ctx>,
    functions: Functions<'ctx>,
}

impl<'ctx> Store<'ctx> {
    pub fn new() -> Self {
        Self {
            variables: Variables::new(),
            functions: Functions::new(),
        }
    }
}

pub struct Compiler<'function, 'ctx> {
    context: &'ctx inkwell::context::Context,
    module: inkwell::module::Module<'ctx>,
    llvm_builder: Builder<'ctx>,
    // scope_depth: usize,
    // loop_details: Vec<(usize, usize)>,
    // globals: GlobalStore,
    function: &'function mut Function,
}

impl<'function, 'ctx> Compiler<'function, 'ctx> {
    pub fn new(
        function: &'function mut Function,
        context: &'ctx inkwell::context::Context,
    ) -> Self {
        let builder = Builder::new(context);

        let compiler = Compiler {
            context,
            function,
            // scope_depth,
            module: context.create_module("vif"),
            llvm_builder: builder,
            // loop_details: Vec::new(),
            // globals: GlobalStore::new(),
        };

        //     compiler.make_global(Global::Native(NativeFunction::new(
        //         NativeFunctionCallee::GetTime,
        //     )));

        //     compiler.make_global(Global::Native(NativeFunction::new(
        //         NativeFunctionCallee::Sleep,
        //     )));
        // };

        compiler
    }

    pub fn add_builtin_functions(&self, store: &mut Store<'ctx>) {
        let printf_type = self.context.i64_type().fn_type(
            &[self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into()],
            true,
        );
        let printf = self.module.add_function("printf", printf_type, None);

        store.functions.add("print".to_owned(), printf);
    }

    pub fn compile(
        &self,
        function: &'function ast::Function,
        store: &mut Store<'ctx>,
    ) -> Result<inkwell::basic_block::BasicBlock<'ctx>, CompilerError> {
        let function_value = self
            .llvm_builder
            .declare_user_function(function, &self.module);

        store
            .functions
            .add(function.name.to_owned(), function_value);

        for (value, param) in function_value
            .get_params()
            .iter()
            .zip(function.params.iter())
        {
            store.variables.add(param.name.to_owned(), value.clone());
        }

        let block = self
            .llvm_builder
            .create_function_block(function_value, "entry");

        // let i8_ptr_type = context.i32_type();
        // let puts_type = i8_ptr_type.fn_type(
        //     &[
        //         context.ptr_type(inkwell::AddressSpace::default()).into(),
        //         context.i32_type().into(),
        //     ],
        //     false,
        // );
        // let puts = module.add_function("puts", puts_type, None);

        // let main_type = context.i32_type().fn_type(&[], false);
        // let main_func = module.add_function("main", main_type, None);

        // let entry_block = context.append_basic_block(main_func, "entry");

        // builder.position_at_end(entry_block);

        // let hello_world = self
        //     .llvm_builder
        //     .declare_global_string("hello_world", "Hello, World!\n");

        // self.llvm_builder
        //     .builder
        //     .build_call(puts, &[hello_world.as_pointer_value().into()], "call_puts")
        //     .map_err(|e| CompilerError::Unknown(format!("LLVM issue: {}", e)))?;

        for token in function.body.iter() {
            self.statement(token, store)?;
        }

        Ok(block)
    }

    pub fn add_return(&self) -> Result<(), CompilerError> {
        self.llvm_builder
            .return_statement(&self.llvm_builder.value_int(1))
    }

    pub fn print_module_to_file(&self, path: &str) -> Result<(), CompilerError> {
        self.module
            .print_to_file(Path::new(path))
            .map_err(|e| CompilerError::LLVM(format!("{e}")))
    }

    // fn emit_op_code(&mut self, op_code: OpCode) {
    //     self.function.chunk.write_chunk(op_code);
    // }

    // fn emit_jump(&mut self, op_code: OpCode) -> usize {
    //     self.emit_op_code(op_code);
    //     self.function.chunk.code.len() - 1
    // }

    // fn emit_global(&mut self, variable: Global) {
    //     self.globals.push(variable);
    //     self.emit_op_code(OpCode::Global(self.globals.len() - 1))
    // }

    // fn patch_jump(&mut self, offset: usize) {
    //     let curr = self.function.chunk.code.len();
    //     match self.function.chunk.code.get_mut(offset) {
    //         Some(OpCode::JumpIfFalse(ref mut i)) => *i = curr - *i - 1,
    //         Some(OpCode::Jump(ref mut i)) => *i = curr - *i - 1,
    //         _ => (),
    //     }
    // }

    pub fn statement(
        &self,
        token: &'function ast::Stmt,
        store: &mut Store<'ctx>,
    ) -> Result<(), CompilerError> {
        log::debug!("Starting statement");
        match token {
            ast::Stmt::Expression(expr) => self.expression_statement(expr, store)?,
            ast::Stmt::Return(ret) => self.return_statement(ret, store)?,
            ast::Stmt::Function(func) => self.function_declaration(func, store)?,
            ast::Stmt::Var(var) => self.var_declaration(var, store)?,
            _ => unreachable!(),
            // ast::Stmt::Block(blocks) => self.block(blocks),
            // ast::Stmt::Condition(cond) => self.if_statement(cond),
            // ast::Stmt::While(whi) => self.while_statement(whi),
            // ast::Stmt::Assert(ass) => self.assert_statement(ass),
        };

        Ok(())
    }

    fn return_statement(
        &self,
        token: &'function ast::Return,
        store: &mut Store<'ctx>,
    ) -> Result<(), CompilerError> {
        let value = self.expression(&token.value, store)?;
        self.llvm_builder
            .return_statement(&value.get_basic_value_enum())
    }

    pub fn call(
        &self,
        token: &'function ast::Call,
        store: &mut Store<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let function_value = self.expression(&token.callee, store)?.get_function_value();
        let mut args = token
            .arguments
            .iter()
            .map(|e| self.expression(e, store).unwrap())
            .map(|e| BasicMetadataValueEnum::from(e.get_basic_value_enum()))
            .collect::<Vec<BasicMetadataValueEnum>>();

        if function_value.get_name().to_str().unwrap() == "printf" {
            let s_fmt = self
                .llvm_builder
                .builder
                .build_global_string_ptr("%s\n", "format_str")
                .unwrap();
            args.insert(
                0,
                BasicMetadataValueEnum::PointerValue(s_fmt.as_pointer_value()),
            )
        }

        let value = self.llvm_builder.call(
            function_value,
            &args,
            function_value.get_name().to_str().unwrap(),
        )?;
        // for arg in token.arguments.iter() {
        //     self.function_parameter(arg)?;
        // }

        // self.emit_op_code(OpCode::Call((
        //     token.arguments.len(),
        //     ItemReference::new(Some(token.callee.span.clone())),
        // )));

        Ok(LLVMValue::BasicValueEnum(
            value
                .or_else(|| {
                    let value = self.context.i64_type().const_int(0, false);
                    Some(BasicValueEnum::IntValue(value))
                })
                .unwrap(),
        ))
    }

    // fn if_statement(&mut self, token: &ast::Condition) -> Result<(), CompilerError> {
    //     log::debug!("Starting if statement");

    //     self.expression(&token.expr)?;
    //     let then_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
    //     self.emit_op_code(OpCode::Pop);
    //     self.statement(&token.then)?;

    //     let else_jump = self.emit_jump(OpCode::Jump(self.function.chunk.code.len()));
    //     self.patch_jump(then_jump);
    //     self.emit_op_code(OpCode::Pop);

    //     if token.r#else.is_some() {
    //         self.statement(token.r#else.as_ref().unwrap())?;
    //     }
    //     self.patch_jump(else_jump);
    //     Ok(())
    // }

    // fn assert_statement(&mut self, token: &ast::Assert) -> Result<(), CompilerError> {
    //     log::debug!("Starting assert statement");
    //     self.expression(&token.value)?;
    //     self.emit_op_code(OpCode::AssertTrue(ItemReference::new(Some(
    //         token.value.span.clone(),
    //     ))));
    //     self.emit_op_code(OpCode::Pop);
    //     Ok(())
    // }

    // fn while_statement(&mut self, token: &ast::While) -> Result<(), CompilerError> {
    //     log::debug!("Starting while statement");
    //     let loop_start = self.function.chunk.code.len();

    //     self.expression(&token.condition)?;

    //     let exit_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
    //     self.loop_details.push((loop_start, exit_jump));
    //     self.emit_op_code(OpCode::Pop);
    //     let res = self.statement(&token.body);
    //     self.loop_details.pop().unwrap();
    //     self.emit_op_code(OpCode::Goto(loop_start));

    //     self.patch_jump(exit_jump);
    //     self.emit_op_code(OpCode::Pop);
    //     res
    // }

    // pub fn break_loop(&mut self) -> Result<(), CompilerError> {
    //     log::debug!("Starting break loop statement");
    //     self.emit_op_code(OpCode::False(ItemReference::new(None))); // fake a false condition
    //     match self.loop_details.last() {
    //         Some(detail) => self.emit_op_code(OpCode::Goto(detail.1)),
    //         None => {
    //             return Err(CompilerError::SyntaxError(format!(
    //                 "Unexpected break statement"
    //             )))
    //         }
    //     }
    //     Ok(())
    // }

    // pub fn continue_loop(&mut self) -> Result<(), CompilerError> {
    //     log::debug!("Starting continue loop statement");
    //     match self.loop_details.last() {
    //         Some(detail) => self.emit_op_code(OpCode::Goto(detail.0)),
    //         None => {
    //             return Err(CompilerError::SyntaxError(format!(
    //                 "Unexpected continue statement"
    //             )))
    //         }
    //     }
    //     Ok(())
    // }

    fn function_declaration(
        &self,
        token: &'function ast::Function,
        store: &mut Store<'ctx>,
    ) -> Result<(), CompilerError> {
        log::debug!("Starting function declaration");

        let previous_block = self.llvm_builder.get_current_block().unwrap();
        if self.function.name != "main" {
            let mut new_store = store.clone();
            self.compile(token, &mut new_store)?;
        } else {
            self.compile(token, store)?;
        }

        self.llvm_builder.set_position_at(previous_block);
        Ok(())

        // let function_block = self.llvm_builder.declare_user_function(token, &self.module);
        // self.function_statement(token)?;
    }

    // fn function_statement(&self, token: &ast::Function) -> Result<(), CompilerError> {
    // let mut compiler = Compiler::new(&mut function, self.context);

    // for variable in token.params.iter() {
    //     compiler.register_function_parameter(Box::new(variable.name.clone()));
    // }

    // compiler.compile(&token)?;

    // let mut globals = compiler.end();
    // std::mem::swap(&mut globals, &mut self.globals);
    // log::debug!("Function compiling terminated");
    // self.emit_global(Global::Function(Box::new(function)));
    // Ok(())
    // }

    // fn block(&mut self, token: &Vec<ast::Stmt>) -> Result<(), CompilerError> {
    //     log::debug!("Block starting");
    //     for block in token.iter() {
    //         self.statement(block)?;
    //     }
    //     Ok(())
    // }

    fn var_declaration(
        &self,
        token: &'function ast::Variable,
        store: &mut Store<'ctx>,
    ) -> Result<(), CompilerError> {
        let value = self.expression(&token.value, store)?;
        let var_ptr = self.llvm_builder.declare_variable(token, value)?;
        store.variables.add(token.name.to_owned(), var_ptr.clone());
        Ok(())
    }

    // fn initialize_variable(&mut self) {
    //     if let Some(var) = self.function.locals.last_mut() {
    //         log::debug!("Initialize variable {var}");
    //         var.depth = Some(self.scope_depth);
    //     }
    // }

    // fn define_variable(&mut self, variable_index: usize) {
    //     log::debug!("Starting define variable");
    //     self.initialize_variable();
    //     self.emit_op_code(OpCode::CreateLocal(variable_index - 1))
    // }

    // fn register_variable(&mut self, name: Box<String>) -> usize {
    //     log::debug!("Register variable {}", name);
    //     let variable = Variable::new(name, None);
    //     self.function.locals.push(variable);
    //     return self.function.locals.len();
    // }

    // fn register_function_parameter(&mut self, variable_name: Box<String>) {
    //     self.function
    //         .locals
    //         .push(Variable::new(variable_name, Some(self.scope_depth)));
    // }

    // fn make_global(&mut self, variable: Global) -> usize {
    //     match self.globals.find(&variable) {
    //         Some(index) => return index,
    //         None => {
    //             self.globals.push(variable);
    //             return self.globals.len() - 1;
    //         }
    //     }
    // }

    fn expression_statement(
        &self,
        token: &'function Box<ast::Expr>,
        store: &mut Store<'ctx>,
    ) -> Result<(), CompilerError> {
        log::debug!("Starting expression statement");
        self.expression(token, store)?;
        Ok(())
    }

    fn expression(
        &self,
        token: &'function Box<ast::Expr>,
        store: &mut Store<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match &token.body {
            ast::ExprBody::Value(t) => {
                self.value(t, ItemReference::new(Some(token.span.clone())), store)
            }
            ast::ExprBody::Binary(t) => self.binary(t, store),
            ast::ExprBody::Call(t) => self.call(t, store),
            _ => unreachable!(),
            // ast::ExprBody::Unary(t) => self.unary(t),
            // ast::ExprBody::Grouping(t) => self.grouping(t),
            // ast::ExprBody::Assign(t) => self.assign(t),
            // ast::ExprBody::Logical(t) => self.logical(t),
            // ast::ExprBody::LoopKeyword(t) => self.loop_keyword(t),
        }
    }

    //     fn function_parameter(&mut self, token: &Box<ast::Expr>) -> Result<(), CompilerError> {
    //         match &token.body {
    //             ast::ExprBody::Binary(t) => self.binary(t),
    //             ast::ExprBody::Unary(t) => self.unary(t),
    //             ast::ExprBody::Grouping(t) => self.grouping(t),
    //             ast::ExprBody::Value(t) => self.value(t, ItemReference::new(Some(token.span.clone()))),
    //             ast::ExprBody::Assign(t) => self.assign(t),
    //             ast::ExprBody::Logical(t) => self.logical(t),
    //             ast::ExprBody::Call(t) => self.call(t),
    //             ast::ExprBody::LoopKeyword(t) => {
    //                 return Err(CompilerError::SyntaxError(format!(
    //                     "{t} not accepted as function parameter"
    //                 )))
    //             }
    //         }
    //     }

    //     fn logical(&mut self, token: &ast::Logical) -> Result<(), CompilerError> {
    //         match token.operator {
    //             ast::LogicalOperator::And => self.and(token),
    //             ast::LogicalOperator::Or => self.or(token),
    //         }
    //     }

    //     fn assign(&mut self, token: &ast::Assign) -> Result<(), CompilerError> {
    //         self.expression(&token.value)?;
    //         self.set_variable(token.name.as_str())
    //     }

    fn value(
        &self,
        token: &'function ast::Value,
        reference: ItemReference,
        store: &mut Store<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match token {
            ast::Value::Integer(i) => {
                Ok(LLVMValue::BasicValueEnum(self.llvm_builder.value_int(*i)))
            }
            ast::Value::Float(f) => {
                Ok(LLVMValue::BasicValueEnum(self.llvm_builder.value_float(*f)))
            }
            ast::Value::True => Ok(LLVMValue::BasicValueEnum(self.llvm_builder.value_int(1))),
            ast::Value::False => Ok(LLVMValue::BasicValueEnum(self.llvm_builder.value_int(0))),
            ast::Value::None => Ok(LLVMValue::BasicValueEnum(self.llvm_builder.value_int(0))),
            ast::Value::Variable(s) => self
                .get_variable(&s, store)
                .or_else(|_| self.get_function(&s, store)),
            ast::Value::String(s) => Ok(LLVMValue::BasicValueEnum(
                self.llvm_builder
                    .global_string("", s)?
                    .as_basic_value_enum(),
            )), // ast::Value::Float(f) => self.emit_global(Global::Float(*f)),
                // ast::Value::True => self.emit_op_code(OpCode::True(reference)),
                // ast::Value::False => self.emit_op_code(OpCode::False(reference)),
                // ast::Value::None => self.emit_op_code(OpCode::None(reference)),
        }
    }

    //     fn loop_keyword(&mut self, token: &ast::LoopKeyword) -> Result<(), CompilerError> {
    //         match token {
    //             ast::LoopKeyword::Break => self.break_loop()?,
    //             ast::LoopKeyword::Continue => self.continue_loop()?,
    //         };

    //         Ok(())
    //     }

    fn binary(
        &self,
        token: &'function ast::Binary,
        store: &mut Store<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let reference = ItemReference::new(Some(token.right.span.clone()));
        let value_left = self.expression(&token.left, store)?;
        let value_right = self.expression(&token.right, store)?;
        self.operator(&token.operator, value_left, value_right, reference, store)
    }

    fn operator(
        &self,
        token: &'function ast::Operator,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
        reference: ItemReference,
        store: &mut Store<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match token {
            ast::Operator::Plus => self.llvm_builder.add(value_left, value_right),
            ast::Operator::Minus => self.llvm_builder.sub(value_left, value_right),
            ast::Operator::Divide => self.llvm_builder.divide(value_left, value_right),
            ast::Operator::Multiply => self.llvm_builder.multiply(value_left, value_right),
            _ => unreachable!(),
            // ast::Operator::BangEqual => OpCode::NotEqual(reference),
            // ast::Operator::Less => OpCode::Less(reference),
            // ast::Operator::LessEqual => OpCode::LessOrEqual(reference),
            // ast::Operator::Greater => OpCode::Greater(reference),
            // ast::Operator::GreaterEqual => OpCode::GreaterOrEqual(reference),
            // ast::Operator::Equal => OpCode::Equal(reference),
            // ast::Operator::Comma => OpCode::NotImplemented,
            // ast::Operator::Modulo => OpCode::Modulo(reference),

            // might have to transform them earlier because we don't know the ptr to update here
            ast::Operator::PlusEqual => {
                let var_name = value_left.get_name();
                let ptr = store.variables.get(var_name).ok_or_else(|| {
                    CompilerError::Unknown(format!("Variable unknown: {}", value_left.get_name()))
                })?;
                let new_value = self.llvm_builder.add(value_left, value_right).unwrap();
                self.llvm_builder.store_value(
                    ptr.ptr.into_pointer_value(),
                    new_value.get_basic_value_enum(),
                )?;
                Ok(new_value)
            }
            ast::Operator::MinusEqual => {
                let var_name = value_left.get_name();
                let ptr = store.variables.get(var_name).ok_or_else(|| {
                    CompilerError::Unknown(format!("Variable unknown: {}", value_left.get_name()))
                })?;
                let new_value = self.llvm_builder.sub(value_left, value_right).unwrap();
                self.llvm_builder.store_value(
                    ptr.ptr.into_pointer_value(),
                    new_value.get_basic_value_enum(),
                )?;
                Ok(new_value)
            }

            ast::Operator::DevideEqual => {
                let var_name = value_left.get_name();
                let ptr = store.variables.get(var_name).ok_or_else(|| {
                    CompilerError::Unknown(format!("Variable unknown: {}", value_left.get_name()))
                })?;
                let new_value = self.llvm_builder.divide(value_left, value_right).unwrap();
                self.llvm_builder.store_value(
                    ptr.ptr.into_pointer_value(),
                    new_value.get_basic_value_enum(),
                )?;
                Ok(new_value)
            }
            ast::Operator::MultiplyEqual => {
                let var_name = value_left.get_name();
                let ptr = store.variables.get(var_name).ok_or_else(|| {
                    CompilerError::Unknown(format!("Variable unknown: {}", value_left.get_name()))
                })?;
                let new_value = self.llvm_builder.multiply(value_left, value_right).unwrap();
                self.llvm_builder.store_value(
                    ptr.ptr.into_pointer_value(),
                    new_value.get_basic_value_enum(),
                )?;
                Ok(new_value)
            }
        }
    }

    //     pub fn and(&mut self, token: &ast::Logical) -> Result<(), CompilerError> {
    //         log::debug!("Starting and operation");
    //         self.expression(&token.left)?;
    //         let end_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
    //         self.emit_op_code(OpCode::Pop);
    //         self.expression(&token.right)?;
    //         self.patch_jump(end_jump);
    //         Ok(())
    //     }

    //     pub fn or(&mut self, token: &ast::Logical) -> Result<(), CompilerError> {
    //         log::debug!("Starting or operation");
    //         self.expression(&token.left)?;
    //         let else_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
    //         let end_jump = self.emit_jump(OpCode::Jump(self.function.chunk.code.len()));

    //         self.patch_jump(else_jump);
    //         self.emit_op_code(OpCode::Pop);

    //         self.expression(&token.right)?;
    //         self.patch_jump(end_jump);
    //         Ok(())
    //     }

    //     pub fn set_variable(&mut self, var_name: &str) -> Result<(), CompilerError> {
    //         log::debug!("Starting variable assignment");

    //         let op_code = match self.resolve_local(&var_name)? {
    //             VariableType::Local(index) => OpCode::SetLocal(index),
    //             VariableType::Inherited(v) => OpCode::SetInheritedLocal(v),
    //             VariableType::Global(i) => OpCode::SetGlobal(i),
    //         };

    //         self.emit_op_code(op_code);
    //         Ok(())
    //     }

    pub fn get_variable(
        &self,
        var_name: &str,
        store: &mut Store<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        log::debug!("Starting variable");

        if let Some(ptr) = store.variables.get(var_name.to_owned()) {
            self.llvm_builder
                .load_variable(var_name, &ptr.ptr, self.context.i64_type())
                .and_then(|v| Ok(LLVMValue::BasicValueEnum(v)))
        } else {
            Err(CompilerError::Unknown(format!(
                "Variable {} not found",
                var_name
            )))
        }

        // let op_code = match self.resolve_local(var_name)? {
        //     VariableType::Local(index) => OpCode::GetLocal(index),
        //     VariableType::Inherited(v) => OpCode::GetInheritedLocal(v),
        //     VariableType::Global(v) => OpCode::GetGlobal(v),
        // };

        // self.emit_op_code(op_code);
    }

    pub fn get_function(
        &self,
        function_name: &str,
        store: &mut Store<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        if let Some(ptr) = store.functions.get(function_name.to_owned()) {
            Ok(LLVMValue::FunctionValue(ptr.ptr))
        } else {
            Err(CompilerError::Unknown(format!(
                "Function {} not found",
                function_name
            )))
        }

        // let op_code = match self.resolve_local(var_name)? {
        //     VariableType::Local(index) => OpCode::GetLocal(index),
        //     VariableType::Inherited(v) => OpCode::GetInheritedLocal(v),
        //     VariableType::Global(v) => OpCode::GetGlobal(v),
        // };

        // self.emit_op_code(op_code);
    }

    //     fn resolve_local(&mut self, var_name: &str) -> Result<VariableType, CompilerError> {
    //         log::debug!("Resolve variable {}", var_name);

    //         for (i, local) in self.function.locals.iter().enumerate().rev() {
    //             if local.name.as_str() != var_name {
    //                 continue;
    //             }

    //             if local.depth.is_none() {
    //                 return Err(CompilerError::Unknown(format!(
    //                     "Can't read local variable in its own initializer: {}",
    //                     var_name
    //                 )));
    //             };

    //             return Ok(VariableType::Local(i));
    //         }

    //         for local in self.function.inherited_locals.iter().rev() {
    //             if local.var_name.as_str() != var_name {
    //                 continue;
    //             };
    //             return Ok(VariableType::Inherited(InheritedLocalPos::new(
    //                 local.pos,
    //                 local.depth,
    //             )));
    //         }

    //         for (i, global) in self.globals.as_vec().iter().enumerate() {
    //             match global {
    //                 Global::Identifier(v) => {
    //                     if v.name.as_str() == var_name {
    //                         return Ok(VariableType::Global(i));
    //                     };
    //                 }
    //                 Global::Native(f) if f.name == var_name => {
    //                     return Ok(VariableType::Global(i));
    //                 }
    //                 _ => continue,
    //             }
    //         }

    //         return Err(CompilerError::ConstantNotFound(format!(
    //             "Unknown variable: {var_name}"
    //         )));
    //     }

    //     fn unary(&mut self, token: &ast::Unary) -> Result<(), CompilerError> {
    //         self.expression(&token.right)?;
    //         self.unary_operator(
    //             &token.operator,
    //             ItemReference::new(Some(token.right.span.clone())),
    //         );
    //         Ok(())
    //     }

    //     fn unary_operator(&mut self, token: &ast::UnaryOperator, reference: ItemReference) {
    //         self.emit_op_code(match token {
    //             ast::UnaryOperator::Minus => OpCode::Negate(reference),
    //             ast::UnaryOperator::Not => OpCode::Not(reference),
    //         })
    //     }

    //     pub fn grouping(&mut self, token: &ast::Grouping) -> Result<(), CompilerError> {
    //         self.expression(&token.expr)
    //     }

    //     pub fn end(mut self) -> GlobalStore {
    //         match self.function.chunk.code.last() {
    //             Some(OpCode::Return(r)) => (),
    //             _ => {
    //                 self.emit_op_code(OpCode::None(ItemReference::new(None)));
    //                 self.emit_op_code(OpCode::Return(ItemReference::new(None)));
    //             }
    //         }
    //         self.globals
    //     }
}
