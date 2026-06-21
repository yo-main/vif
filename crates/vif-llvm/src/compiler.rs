use crate::builder::Builder;
use crate::error::CompilerError;

use inkwell;
use inkwell::basic_block::BasicBlock;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::targets::FileType;
use inkwell::targets::InitializationConfig;
use inkwell::targets::Target;
use inkwell::targets::TargetMachine;
use inkwell::values::BasicMetadataValueEnum;
use inkwell::values::BasicValue;
use std::collections::HashMap;

use vif_typing::Assign;
use vif_typing::Binary;
use vif_typing::Call;
use vif_typing::Callable;
use vif_typing::Condition;
use vif_typing::Entrypoint;
use vif_typing::Expr;
use vif_typing::ExprBody;
use vif_typing::Function;
use vif_typing::Logical;
use vif_typing::LogicalOperator;
use vif_typing::LoopKeyword;
use vif_typing::Operator;
use vif_typing::Return;
use vif_typing::Span;
use vif_typing::Stmt;
use vif_typing::Type;
use vif_typing::Unary;
use vif_typing::UnaryOperator;
use vif_typing::Value;
use vif_typing::Variable;
use vif_typing::While;

use crate::builder::LLVMValue;
use vif_loader::log;

#[derive(PartialEq, Debug)]
pub struct ItemReference {
    span: Option<Span>,
}

impl ItemReference {
    pub fn new(span: Option<Span>) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone)]
struct Variables<'ctx> {
    data: HashMap<String, LLVMValue<'ctx>>,
}

impl<'ctx> Variables<'ctx> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, var_name: String, value: LLVMValue<'ctx>) {
        self.data.insert(var_name, value);
    }

    fn get(&self, var_name: String) -> Option<&LLVMValue<'ctx>> {
        self.data.get(&var_name)
    }
}

#[derive(Debug, Clone)]
struct Functions<'ctx> {
    data: HashMap<String, LLVMValue<'ctx>>,
}

impl<'ctx, 'function> Functions<'ctx> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, var_name: String, value: LLVMValue<'ctx>) {
        self.data.insert(var_name, value);
    }

    fn get(&self, var_name: String) -> Option<&LLVMValue<'ctx>> {
        self.data.get(&var_name)
    }
}

#[derive(Debug, Clone)]
struct LoopContext<'ctx> {
    cond: inkwell::basic_block::BasicBlock<'ctx>,
    end: inkwell::basic_block::BasicBlock<'ctx>,
}

impl<'ctx> LoopContext<'ctx> {
    fn new(
        cond: inkwell::basic_block::BasicBlock<'ctx>,
        end: inkwell::basic_block::BasicBlock<'ctx>,
    ) -> Self {
        Self { cond, end }
    }
}

#[derive(Debug, Clone)]
pub struct CompilerContext<'ctx> {
    return_as_pointer: bool,
    loop_context: Vec<LoopContext<'ctx>>,
    variables: Variables<'ctx>,
    functions: Functions<'ctx>,
}

impl<'ctx> CompilerContext<'ctx> {
    pub fn new() -> Self {
        Self {
            return_as_pointer: false,
            variables: Variables::new(),
            functions: Functions::new(),
            loop_context: Vec::new(),
        }
    }
}

pub struct Compiler<'ctx> {
    context: &'ctx inkwell::context::Context,
    module: inkwell::module::Module<'ctx>,
    llvm_builder: Builder<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx inkwell::context::Context) -> Self {
        let builder = Builder::new(context);

        let compiler = Compiler {
            context,
            module: context.create_module("vif"),
            llvm_builder: builder,
        };

        compiler
    }

    pub fn add_builtin_functions(&self, context: &mut CompilerContext<'ctx>) {
        let print_type = self.context.i64_type().fn_type(
            &[self
                .context
                .ptr_type(inkwell::AddressSpace::default())
                .into()],
            true,
        );
        let print = self.module.add_function("printf", print_type, None);

        context.functions.add(
            "print".to_owned(),
            LLVMValue::new_function(
                print,
                Type::Callable(Callable::new_infinite(Box::new(Type::None))),
            ),
        );
    }

    pub fn compile_entrypoint(
        &self,
        entrypoint: &Entrypoint,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<BasicBlock<'ctx>, CompilerError> {
        let function_value = self
            .llvm_builder
            .declare_entrypoint("__entrypoint", &self.module);

        context
            .functions
            .add("__entrypoint".to_owned(), function_value.clone());

        // for (value, param) in function_value
        //     .get_function_value()
        //     .get_function_parameters()
        //     .iter()
        //     .zip(entrypoint.params.iter())
        // {
        //     context.variables.add(
        //         param.name.to_owned(),
        //         LLVMValue::new_variable(value.clone(), param.typing.clone()),
        //     );
        // }

        let entry_block = self
            .llvm_builder
            .create_function_block(&function_value, "entry");

        for token in entrypoint.body.iter() {
            self.statement(token, context)?;
        }

        self.llvm_builder.return_statement(&LLVMValue::new_value(
            self.llvm_builder.value_int(1),
            Type::Int,
        ))?;

        Ok(entry_block)
    }

    pub fn compile_function(
        &self,
        function: &Function,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<BasicBlock<'ctx>, CompilerError> {
        let function_value = self
            .llvm_builder
            .declare_user_function(function, &self.module);

        context
            .functions
            .add(function.name.to_owned(), function_value.clone());

        for (value, param) in function_value
            .get_function_value()
            .get_function_parameters()
            .iter()
            .zip(function.params.iter())
        {
            context.variables.add(
                param.name.to_owned(),
                LLVMValue::new_variable(value.clone(), param.typing.clone()),
            );
        }

        let entry_block = self
            .llvm_builder
            .create_function_block(&function_value, "entry");

        for token in function.body.iter() {
            self.statement(token, context)?;
        }

        Ok(entry_block)
    }

    pub fn add_return_none(&self) -> Result<(), CompilerError> {
        self.llvm_builder.return_statement(&LLVMValue::new_value(
            self.llvm_builder.value_bool(false),
            Type::None,
        ))
    }

    pub fn as_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn execute(&self) -> Result<(), CompilerError> {
        let engine = self
            .module
            .create_jit_execution_engine(inkwell::OptimizationLevel::Aggressive)
            .map_err(|_| CompilerError::LLVM("Could not start JIT engine".to_owned()))?;

        let function = self.module.get_function("__entrypoint").unwrap();

        unsafe { engine.run_function(function, &[]) };

        Ok(())
    }

    pub fn build_binary(&self, filename: &str) -> Result<(), CompilerError> {
        let code = self.module.print_to_string();
        let buffer =
            MemoryBuffer::create_from_memory_range(code.to_str().unwrap().as_bytes(), "here");

        let ctx = inkwell::context::Context::create();
        let new_module = ctx.create_module_from_ir(buffer).unwrap();

        Target::initialize_all(&InitializationConfig::default());
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).expect("Failed to get target");
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                inkwell::OptimizationLevel::Aggressive,
                inkwell::targets::RelocMode::PIC,
                inkwell::targets::CodeModel::Default,
            )
            .expect("Failed to create target machine");

        target_machine
            .write_to_file(
                &new_module,
                FileType::Object,
                std::path::Path::new(filename),
            )
            .expect("Failed to write object file");

        Ok(())
    }

    pub fn statement(
        &self,
        token: &Stmt,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        log::debug!("Starting statement");
        match token {
            Stmt::Expression(expr) => self.expression_statement(expr, context)?,
            Stmt::Return(ret) => self.return_statement(ret, context)?,
            Stmt::Function(func) => self.function_declaration(func, context)?,
            Stmt::Var(var) => self.var_declaration(var, context)?,
            Stmt::Condition(cond) => self.if_statement(cond, context)?,
            Stmt::Block(blocks) => self.block(blocks, context)?,
            Stmt::While(whi) => self.while_statement(whi, context)?,
            Stmt::Assert(_) => unimplemented!(), // TODO!!! self.assert_statement(ass),
        };

        Ok(())
    }

    fn return_statement(
        &self,
        token: &Return,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        let value = self.expression(&token.value, context)?;
        if context.return_as_pointer && value.is_value() {
            let temp_var = self.llvm_builder.allocate_and_store_value(
                value.as_value(),
                "",
                value.get_typing(),
            )?;
            self.llvm_builder.return_statement(&temp_var)
        } else if !context.return_as_pointer && value.is_variable() {
            let temp_var = LLVMValue::new_value(
                self.llvm_builder.load_llvm_value("", &value)?,
                value.get_typing(),
            );
            self.llvm_builder.return_statement(&temp_var)
        } else {
            self.llvm_builder.return_statement(&value)
        }
    }

    pub fn call(
        &self,
        token: &Call,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let function_value = self.expression(&token.callee, context)?;

        let mut args;

        if function_value.get_name() == "printf" {
            let mut str_fmt = String::new();
            args = token
                .arguments
                .iter()
                .map(|e| {
                    let value = self.expression(e, context).unwrap();
                    str_fmt.push_str(value.get_typing().printf_formatter());
                    value
                })
                .map(|e| {
                    BasicMetadataValueEnum::from(
                        self.llvm_builder.load_llvm_value("", &e.clone()).unwrap(),
                    )
                })
                .collect::<Vec<BasicMetadataValueEnum>>();
            str_fmt.push_str("\n");
            let s_fmt = self
                .llvm_builder
                .builder
                .build_global_string_ptr(str_fmt.as_str(), "format_str")
                .unwrap();
            args.insert(
                0,
                BasicMetadataValueEnum::PointerValue(s_fmt.as_pointer_value()),
            )
        } else {
            args = token
                .arguments
                .iter()
                .map(|e| self.expression(e, context).unwrap())
                .map(|e| match e {
                    LLVMValue::RawValue(_) => self
                        .llvm_builder
                        .allocate_and_store_value(e.as_value(), "", e.get_typing())
                        .unwrap(),
                    o => o,
                })
                .map(|e| BasicMetadataValueEnum::from(e.get_variable().get_basic_value_enum()))
                .collect::<Vec<BasicMetadataValueEnum>>();
        }

        self.llvm_builder.call(
            function_value.get_function_value(),
            &args,
            function_value.get_name().as_str(),
        )
    }

    fn if_statement(
        &self,
        token: &Condition,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        let expression = self.expression(&token.expr, context)?;

        let current_block = self.llvm_builder.get_current_block().unwrap();
        let end_block = self.llvm_builder.create_block("end");
        let then_block = self.llvm_builder.create_block("then");
        let mut else_block = None;

        self.llvm_builder.set_position_at(then_block);
        self.statement(&token.then, context)?;
        self.llvm_builder.goto_block(end_block)?;

        if token.r#else.is_some() {
            else_block = Some(self.llvm_builder.create_block("else"));
            self.llvm_builder.set_position_at(else_block.unwrap());
            self.statement(token.r#else.as_ref().unwrap(), context)?;
            self.llvm_builder.goto_block(end_block)?;
        }

        self.llvm_builder.set_position_at(current_block);

        self.llvm_builder.create_branche(
            expression,
            then_block,
            else_block.unwrap_or(end_block),
        )?;

        self.llvm_builder.set_position_at(end_block);

        // self.patch_jump(else_jump);
        Ok(())
    }

    // fn assert_statement(
    //     &self,
    //     token: &ast::Assert,
    //     store: &mut Store<'ctx>,
    // ) -> Result<LLVMValue<'ctx>, CompilerError> {
    //     log::debug!("Starting assert statement");
    //     self.expression(&token.value)?;
    //     self.emit_op_code(OpCode::AssertTrue(ItemReference::new(Some(
    //         token.value.span.clone(),
    //     ))));
    //     self.emit_op_code(OpCode::Pop);
    //     Ok(())
    // }

    fn while_statement(
        &self,
        token: &While,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        log::debug!("Starting while statement");

        let cond_block = self.llvm_builder.create_block("cond");
        let end_block = self.llvm_builder.create_block("end");
        let loop_block = self.llvm_builder.create_block("loop");

        context
            .loop_context
            .push(LoopContext::new(cond_block.clone(), end_block.clone()));

        self.llvm_builder.goto_block(cond_block)?;

        self.llvm_builder.set_position_at(cond_block);
        let cond = self.expression(&token.condition, context)?;
        self.llvm_builder
            .create_branche(cond, loop_block, end_block)?;

        self.llvm_builder.set_position_at(loop_block);
        self.statement(&token.body, context)?;
        self.llvm_builder.goto_block(cond_block)?;

        self.llvm_builder.set_position_at(end_block);

        _ = context.loop_context.pop();

        Ok(())
    }

    fn function_declaration(
        &self,
        token: &Function,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        log::debug!("Starting function declaration");

        let previous_block = self.llvm_builder.get_current_block().unwrap();

        if token.name != "main" {
            let mut new_context = context.clone();
            new_context.return_as_pointer = false;

            self.compile_function(token, &mut new_context)?;
            context.functions.add(
                token.name.clone(),
                new_context
                    .functions
                    .get(token.name.clone())
                    .unwrap()
                    .clone(),
            );
        } else {
            self.compile_function(token, context)?;
        }

        let last_block = self.llvm_builder.get_current_block().unwrap();
        if let None = last_block.get_terminator() {
            self.add_return_none()?;
        }

        self.llvm_builder.set_position_at(previous_block);
        Ok(())
    }

    fn block(
        &self,
        token: &Vec<Stmt>,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        for stmt in token.iter() {
            self.statement(stmt, context)?;
        }

        Ok(())
    }

    fn var_declaration(
        &self,
        token: &Variable,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        let value = self.expression(&token.value, context)?;
        let var_ptr = self.llvm_builder.declare_variable(token, value)?;
        context.variables.add(token.name.to_owned(), var_ptr);
        Ok(())
    }

    fn expression_statement(
        &self,
        token: &Expr,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<(), CompilerError> {
        log::debug!("Starting expression statement");
        self.expression(token, context)?;
        Ok(())
    }

    fn expression(
        &self,
        token: &Expr,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match &token.body {
            ExprBody::Value(t) => {
                self.value(t, ItemReference::new(Some(token.span.clone())), context)
            }
            ExprBody::Binary(t) => self.binary(t, context),
            ExprBody::Call(t) => self.call(t, context),
            ExprBody::Assign(t) => self.assign(t, context),
            ExprBody::Unary(t) => self.unary(t, context),
            ExprBody::Logical(t) => self.logical(t, context),
            ExprBody::LoopKeyword(t) => self.loop_keyword(t, context),
        }
    }

    fn logical(
        &self,
        token: &Logical,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match token.operator {
            LogicalOperator::And => self.and(token, context),
            LogicalOperator::Or => self.or(token, context),
        }
    }

    fn and(
        &self,
        token: &Logical,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let expr1 = self.expression(&token.left, context)?;
        let expr2 = self.expression(&token.right, context)?;

        let expr1_is_true = self.llvm_builder.is_truthy(expr1)?;
        let expr2_is_true = self.llvm_builder.is_truthy(expr2)?;

        self.llvm_builder.and(expr1_is_true, expr2_is_true)
    }

    fn or(
        &self,
        token: &Logical,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let first_block = self.llvm_builder.create_block("first");
        let second_block = self.llvm_builder.create_block("second");
        let merge_block = self.llvm_builder.create_block("merge");

        let expression1 = self.expression(&token.left, context)?;
        let value = self.llvm_builder.allocate(expression1.clone())?;

        let expression_1_truthy = self.llvm_builder.is_truthy(expression1.clone())?;

        self.llvm_builder
            .create_branche(expression_1_truthy, first_block, second_block)?;

        self.llvm_builder.set_position_at(first_block);
        self.llvm_builder
            .store_value(value, expression1.get_basic_value_enum())?;
        self.llvm_builder.goto_block(merge_block)?;

        self.llvm_builder.set_position_at(second_block);
        let expression2 = self.expression(&token.right, context)?;
        self.llvm_builder
            .store_value(value, expression2.get_basic_value_enum())?;
        self.llvm_builder.goto_block(merge_block)?;

        self.llvm_builder.set_position_at(merge_block);

        let result = self.llvm_builder.load_llvm_value(
            "",
            &LLVMValue::new_variable(value, expression1.get_typing()),
        )?;

        Ok(LLVMValue::new_value(result, expression1.get_typing()))
    }

    fn assign(
        &self,
        token: &Assign,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let expr = self.expression(&token.value, context)?;
        let variable = context.variables.get(token.name.clone()).unwrap();

        match &expr {
            LLVMValue::RawValue(_) => self
                .llvm_builder
                .store_value(variable.as_pointer(), expr.as_value())?,
            LLVMValue::Variable(_) => self.llvm_builder.store_value(
                variable.as_pointer(),
                expr.as_pointer().as_basic_value_enum(),
            )?,
            LLVMValue::Function(_) => self.llvm_builder.store_value(
                variable.as_pointer(),
                expr.as_pointer().as_basic_value_enum(),
            )?,
        };

        // assignment does not produce anything
        Ok(LLVMValue::new_value(
            self.llvm_builder.value_bool(false),
            Type::None,
        ))
    }

    fn value(
        &self,
        token: &Value,
        reference: ItemReference,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match token {
            Value::Integer(i) => Ok(LLVMValue::new_value(
                self.llvm_builder.value_int(*i),
                Type::Int,
            )),
            Value::Float(f) => Ok(LLVMValue::new_value(
                self.llvm_builder.value_float(*f),
                Type::Float,
            )),
            Value::True => Ok(LLVMValue::new_value(
                self.llvm_builder.value_bool(true),
                Type::Bool,
            )),
            Value::False => Ok(LLVMValue::new_value(
                self.llvm_builder.value_bool(false),
                Type::Bool,
            )),
            Value::None => Ok(LLVMValue::new_value(
                self.llvm_builder.value_bool(false),
                Type::None,
            )),
            Value::Variable(s) => self
                .get_variable(&s, context)
                .or_else(|_| self.get_function(&s, context)),
            Value::String(s) => Ok(LLVMValue::new_variable(
                self.llvm_builder.global_string("", s)?.as_pointer_value(),
                Type::String,
            )),
        }
    }

    fn loop_keyword(
        &self,
        token: &LoopKeyword,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match token {
            LoopKeyword::Break => self
                .llvm_builder
                .goto_block(context.loop_context.last().unwrap().end)?,
            LoopKeyword::Continue => self
                .llvm_builder
                .goto_block(context.loop_context.last().unwrap().cond)?,
        };

        // this doesn't return any value
        Ok(LLVMValue::new_value(
            self.llvm_builder.value_bool(false),
            Type::None,
        ))
    }

    fn binary(
        &self,
        token: &Binary,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let reference = ItemReference::new(Some(token.right.span.clone()));
        let value_left = self.expression(&token.left, context)?;
        let value_right = self.expression(&token.right, context)?;
        self.operator(&token.operator, value_left, value_right, reference, context)
    }

    fn operator(
        &self,
        token: &Operator,
        value_left: LLVMValue<'ctx>,
        value_right: LLVMValue<'ctx>,
        reference: ItemReference,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        match token {
            Operator::Plus => self.llvm_builder.add(value_left, value_right),
            Operator::Minus => self.llvm_builder.sub(value_left, value_right),
            Operator::Divide => self.llvm_builder.divide(value_left, value_right),
            Operator::Multiply => self.llvm_builder.multiply(value_left, value_right),
            Operator::EqualEqual => self.llvm_builder.equal(value_left, value_right),
            Operator::Greater => self.llvm_builder.greater(value_left, value_right),
            Operator::GreaterEqual => self.llvm_builder.greater_or_equal(value_left, value_right),
            Operator::BangEqual => self.llvm_builder.not_equal(value_left, value_right),
            Operator::Less => self.llvm_builder.less(value_left, value_right),
            Operator::LessEqual => self.llvm_builder.less_or_equal(value_left, value_right),
            Operator::Modulo => self.llvm_builder.modulo(value_left, value_right),
        }
    }

    pub fn get_variable(
        &self,
        var_name: &str,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        log::debug!("Starting variable");

        if let Some(ptr) = context.variables.get(var_name.to_owned()) {
            Ok(ptr.clone())
        } else {
            Err(CompilerError::Unknown(format!(
                "Variable {} not found",
                var_name
            )))
        }
    }

    pub fn get_function(
        &self,
        function_name: &str,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        if let Some(ptr) = context.functions.get(function_name.to_owned()) {
            Ok(ptr.clone())
        } else {
            Err(CompilerError::Unknown(format!(
                "Function {} not found",
                function_name
            )))
        }
    }

    fn unary(
        &self,
        token: &Unary,
        context: &mut CompilerContext<'ctx>,
    ) -> Result<LLVMValue<'ctx>, CompilerError> {
        let expr = self.expression(&token.right, context)?;
        match token.operator {
            UnaryOperator::Minus => self.llvm_builder.create_neg(expr),
            UnaryOperator::Not => self.llvm_builder.create_not(expr),
        }
    }
}
