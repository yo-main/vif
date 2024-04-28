use crate::error::CompilerError;
use crate::Global;
use crate::NativeFunction;
use crate::NativeFunctionCallee;
use crate::OpCode;
use vif_loader::log;
use vif_objects::ast;
use vif_objects::function::Arity;
use vif_objects::function::Function;
use vif_objects::global_store::GlobalStore;
use vif_objects::op_code::Return;
use vif_objects::variable::InheritedLocalPos;
use vif_objects::variable::InheritedVariable;
use vif_objects::variable::Variable;
use vif_objects::variable::VariableType;

pub struct Compiler<'function> {
    scope_depth: usize,
    loop_details: Vec<(usize, usize)>,
    globals: GlobalStore,
    function: &'function mut Function,
}

impl<'function> Compiler<'function> {
    pub fn new(function: &'function mut Function, scope_depth: usize) -> Self {
        let mut compiler = Compiler {
            function,
            scope_depth,
            loop_details: Vec::new(),
            globals: GlobalStore::new(),
        };

        if scope_depth == 0 {
            compiler.make_global(Global::Native(NativeFunction::new(
                NativeFunctionCallee::GetTime,
            )));
            compiler.make_global(Global::Native(NativeFunction::new(
                NativeFunctionCallee::Print,
            )));
            compiler.make_global(Global::Native(NativeFunction::new(
                NativeFunctionCallee::Sleep,
            )));
        };

        compiler
    }

    pub fn compile(&mut self, function: &ast::Function) -> Result<(), CompilerError> {
        for token in function.body.iter() {
            self.statement(token)?;
        }
        Ok(())
    }

    fn emit_op_code(&mut self, op_code: OpCode) {
        self.function.chunk.write_chunk(op_code);
    }

    fn emit_jump(&mut self, op_code: OpCode) -> usize {
        self.emit_op_code(op_code);
        self.function.chunk.code.len() - 1
    }

    fn emit_global(&mut self, variable: Global) {
        self.globals.push(variable);
        self.emit_op_code(OpCode::Global(self.globals.len() - 1))
    }

    fn patch_jump(&mut self, offset: usize) {
        let curr = self.function.chunk.code.len();
        match self.function.chunk.code.get_mut(offset) {
            Some(OpCode::JumpIfFalse(ref mut i)) => *i = curr - *i - 1,
            Some(OpCode::Jump(ref mut i)) => *i = curr - *i - 1,
            _ => (),
        }
    }

    pub fn statement(&mut self, token: &ast::Stmt) -> Result<(), CompilerError> {
        log::debug!("Starting statement");
        match token {
            ast::Stmt::Expression(expr) => self.expression_statement(expr),
            ast::Stmt::Return(ret) => self.return_statement(ret),
            ast::Stmt::Block(blocks) => self.block(blocks),
            ast::Stmt::Condition(cond) => self.if_statement(cond),
            ast::Stmt::While(whi) => self.while_statement(whi),
            ast::Stmt::Function(func) => self.function_declaration(func),
            ast::Stmt::Var(var) => self.var_declaration(var),
            ast::Stmt::Assert(ass) => self.assert_statement(ass),
        }
    }

    fn return_statement(&mut self, token: &ast::Return) -> Result<(), CompilerError> {
        self.expression(&token.value)?;
        self.emit_op_code(OpCode::Return(Return::new(Some(token.value.span.clone()))));
        Ok(())
    }

    pub fn call(&mut self, token: &ast::Call) -> Result<(), CompilerError> {
        log::debug!("Starting call");

        self.expression(&token.callee)?;

        for arg in token.arguments.iter() {
            self.function_parameter(arg)?;
        }
        self.emit_op_code(OpCode::Call(token.arguments.len()));
        Ok(())
    }

    fn if_statement(&mut self, token: &ast::Condition) -> Result<(), CompilerError> {
        log::debug!("Starting if statement");

        self.expression(&token.expr)?;
        let then_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        self.emit_op_code(OpCode::Pop);
        self.statement(&token.then)?;

        let else_jump = self.emit_jump(OpCode::Jump(self.function.chunk.code.len()));
        self.patch_jump(then_jump);
        self.emit_op_code(OpCode::Pop);

        if token.r#else.is_some() {
            self.statement(token.r#else.as_ref().unwrap())?;
        }
        self.patch_jump(else_jump);
        Ok(())
    }

    fn assert_statement(&mut self, token: &ast::Assert) -> Result<(), CompilerError> {
        log::debug!("Starting assert statement");
        self.expression(&token.value)?;
        self.emit_op_code(OpCode::AssertTrue);
        self.emit_op_code(OpCode::Pop);
        Ok(())
    }

    fn while_statement(&mut self, token: &ast::While) -> Result<(), CompilerError> {
        log::debug!("Starting while statement");
        let loop_start = self.function.chunk.code.len();

        self.expression(&token.condition)?;

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        self.loop_details.push((loop_start, exit_jump));
        self.emit_op_code(OpCode::Pop);
        let res = self.statement(&token.body);
        self.loop_details.pop().unwrap();
        self.emit_op_code(OpCode::Goto(loop_start));

        self.patch_jump(exit_jump);
        self.emit_op_code(OpCode::Pop);
        res
    }

    pub fn break_loop(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting break loop statement");
        self.emit_op_code(OpCode::False); // fake a false condition
        match self.loop_details.last() {
            Some(detail) => self.emit_op_code(OpCode::Goto(detail.1)),
            None => {
                return Err(CompilerError::SyntaxError(format!(
                    "Unexpected break statement"
                )))
            }
        }
        Ok(())
    }

    pub fn continue_loop(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting continue loop statement");
        match self.loop_details.last() {
            Some(detail) => self.emit_op_code(OpCode::Goto(detail.0)),
            None => {
                return Err(CompilerError::SyntaxError(format!(
                    "Unexpected continue statement"
                )))
            }
        }
        Ok(())
    }

    fn function_declaration(&mut self, token: &ast::Function) -> Result<(), CompilerError> {
        log::debug!("Starting function declaration");
        let index = self.register_variable(Box::new(token.name.clone()));
        self.function_statement(token)?;
        self.define_variable(index);
        Ok(())
    }

    fn function_statement(&mut self, token: &ast::Function) -> Result<(), CompilerError> {
        log::debug!("Starting function statement");
        let arity = if self.scope_depth > 0 {
            token.params.len()
        } else {
            token.params.len()
        };

        let mut function = Function::new(Arity::Fixed(arity), token.name.clone());

        function.locals.push(Variable::new(
            Box::new(token.name.clone()),
            Some(self.scope_depth),
        ));

        for (i, local) in self.function.locals.iter().enumerate() {
            function.inherited_locals.push(InheritedVariable {
                var_name: local.name.clone(),
                depth: self.scope_depth,
                pos: i + 1,
            });
        }

        for v in self.function.inherited_locals.iter() {
            if function
                .inherited_locals
                .iter()
                .any(|x| x.var_name == v.var_name)
            {
                continue;
            }
            function.inherited_locals.push(InheritedVariable {
                var_name: v.var_name.clone(),
                depth: v.depth,
                pos: v.pos,
            })
        }

        let mut compiler = Compiler::new(&mut function, self.scope_depth + 1);
        std::mem::swap(&mut compiler.globals, &mut self.globals);

        for variable in token.params.iter() {
            compiler.register_function_parameter(Box::new(variable.name.clone()));
        }

        log::debug!("Function compiling starting");
        compiler.compile(&token)?;

        let mut globals = compiler.end();
        std::mem::swap(&mut globals, &mut self.globals);
        log::debug!("Function compiling terminated");
        self.emit_global(Global::Function(Box::new(function)));
        Ok(())
    }

    fn block(&mut self, token: &Vec<ast::Stmt>) -> Result<(), CompilerError> {
        log::debug!("Block starting");
        for block in token.iter() {
            self.statement(block)?;
        }
        Ok(())
    }

    fn var_declaration(&mut self, token: &ast::Variable) -> Result<(), CompilerError> {
        log::debug!("Starting variable declaration");
        let index = self.register_variable(Box::new(token.name.to_owned()));
        self.expression(&token.value)?;
        self.define_variable(index);
        Ok(())
    }

    fn initialize_variable(&mut self) {
        if let Some(var) = self.function.locals.last_mut() {
            log::debug!("Initialize variable {var}");
            var.depth = Some(self.scope_depth);
        }
    }

    fn define_variable(&mut self, variable_index: usize) {
        log::debug!("Starting define variable");
        self.initialize_variable();
        self.emit_op_code(OpCode::CreateLocal(variable_index - 1))
    }

    fn register_variable(&mut self, name: Box<String>) -> usize {
        log::debug!("Register variable {}", name);
        let variable = Variable::new(name, None);
        self.function.locals.push(variable);
        return self.function.locals.len();
    }

    fn register_function_parameter(&mut self, variable_name: Box<String>) {
        self.function
            .locals
            .push(Variable::new(variable_name, Some(self.scope_depth)));
    }

    fn make_global(&mut self, variable: Global) -> usize {
        match self.globals.find(&variable) {
            Some(index) => return index,
            None => {
                self.globals.push(variable);
                return self.globals.len() - 1;
            }
        }
    }

    fn expression_statement(&mut self, token: &Box<ast::Expr>) -> Result<(), CompilerError> {
        log::debug!("Starting expression statement");
        self.expression(token)?;
        self.emit_op_code(OpCode::Pop);
        Ok(())
    }

    fn expression(&mut self, token: &Box<ast::Expr>) -> Result<(), CompilerError> {
        match &token.body {
            ast::ExprBody::Binary(t) => self.binary(t),
            ast::ExprBody::Unary(t) => self.unary(t),
            ast::ExprBody::Grouping(t) => self.grouping(t),
            ast::ExprBody::Value(t) => self.value(t),
            ast::ExprBody::Assign(t) => self.assign(t),
            ast::ExprBody::Logical(t) => self.logical(t),
            ast::ExprBody::Call(t) => self.call(t),
            ast::ExprBody::LoopKeyword(t) => self.loop_keyword(t),
        }
    }

    fn function_parameter(&mut self, token: &Box<ast::Expr>) -> Result<(), CompilerError> {
        match &token.body {
            ast::ExprBody::Binary(t) => self.binary(t),
            ast::ExprBody::Unary(t) => self.unary(t),
            ast::ExprBody::Grouping(t) => self.grouping(t),
            ast::ExprBody::Value(t) => self.value(t),
            ast::ExprBody::Assign(t) => self.assign(t),
            ast::ExprBody::Logical(t) => self.logical(t),
            ast::ExprBody::Call(t) => self.call(t),
            ast::ExprBody::LoopKeyword(t) => {
                return Err(CompilerError::SyntaxError(format!(
                    "{t} not accepted as function parameter"
                )))
            }
        }
    }

    fn logical(&mut self, token: &ast::Logical) -> Result<(), CompilerError> {
        match token.operator {
            ast::LogicalOperator::And => self.and(token),
            ast::LogicalOperator::Or => self.or(token),
        }
    }

    fn assign(&mut self, token: &ast::Assign) -> Result<(), CompilerError> {
        self.expression(&token.value)?;
        self.set_variable(token.name.as_str())
    }

    fn value(&mut self, token: &ast::Value) -> Result<(), CompilerError> {
        match token {
            ast::Value::String(s) => self.emit_global(Global::String(Box::new(s.clone()))),
            ast::Value::Integer(i) => self.emit_global(Global::Integer(*i)),
            ast::Value::Float(f) => self.emit_global(Global::Float(*f)),
            ast::Value::Variable(s) => self.get_variable(&s)?,
            ast::Value::True => self.emit_op_code(OpCode::True),
            ast::Value::False => self.emit_op_code(OpCode::False),
            ast::Value::None => self.emit_op_code(OpCode::None),
        };

        Ok(())
    }

    fn loop_keyword(&mut self, token: &ast::LoopKeyword) -> Result<(), CompilerError> {
        match token {
            ast::LoopKeyword::Break => self.break_loop()?,
            ast::LoopKeyword::Continue => self.continue_loop()?,
        };

        Ok(())
    }

    fn binary(&mut self, token: &ast::Binary) -> Result<(), CompilerError> {
        self.expression(&token.left)?;
        self.expression(&token.right)?;
        self.operator(&token.operator);
        Ok(())
    }

    fn operator(&mut self, token: &ast::Operator) {
        self.emit_op_code(match token {
            ast::Operator::Plus => OpCode::Add,
            ast::Operator::Minus => OpCode::Substract,
            ast::Operator::Divide => OpCode::Divide,
            ast::Operator::Multiply => OpCode::Multiply,
            ast::Operator::PlusEqual => OpCode::NotImplemented,
            ast::Operator::MinusEqual => OpCode::NotImplemented,
            ast::Operator::DevideEqual => OpCode::NotImplemented,
            ast::Operator::MultiplyEqual => OpCode::NotImplemented,
            ast::Operator::BangEqual => OpCode::NotEqual,
            ast::Operator::Less => OpCode::Less,
            ast::Operator::LessEqual => OpCode::LessOrEqual,
            ast::Operator::Greater => OpCode::Greater,
            ast::Operator::GreaterEqual => OpCode::GreaterOrEqual,
            ast::Operator::Equal => OpCode::Equal,
            ast::Operator::Comma => OpCode::NotImplemented,
            ast::Operator::Modulo => OpCode::Modulo,
        })
    }

    pub fn and(&mut self, token: &ast::Logical) -> Result<(), CompilerError> {
        log::debug!("Starting and operation");
        self.expression(&token.left)?;
        let end_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        self.emit_op_code(OpCode::Pop);
        self.expression(&token.right)?;
        self.patch_jump(end_jump);
        Ok(())
    }

    pub fn or(&mut self, token: &ast::Logical) -> Result<(), CompilerError> {
        log::debug!("Starting or operation");
        self.expression(&token.left)?;
        let else_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        let end_jump = self.emit_jump(OpCode::Jump(self.function.chunk.code.len()));

        self.patch_jump(else_jump);
        self.emit_op_code(OpCode::Pop);

        self.expression(&token.right)?;
        self.patch_jump(end_jump);
        Ok(())
    }

    pub fn set_variable(&mut self, var_name: &str) -> Result<(), CompilerError> {
        log::debug!("Starting variable assignment");

        let op_code = match self.resolve_local(&var_name)? {
            VariableType::Local(index) => OpCode::SetLocal(index),
            VariableType::Inherited(v) => OpCode::SetInheritedLocal(v),
            VariableType::Global(i) => OpCode::SetGlobal(i),
        };

        self.emit_op_code(op_code);
        Ok(())
    }

    pub fn get_variable(&mut self, var_name: &str) -> Result<(), CompilerError> {
        log::debug!("Starting variable");

        let op_code = match self.resolve_local(var_name)? {
            VariableType::Local(index) => OpCode::GetLocal(index),
            VariableType::Inherited(v) => OpCode::GetInheritedLocal(v),
            VariableType::Global(v) => OpCode::GetGlobal(v),
        };

        self.emit_op_code(op_code);
        Ok(())
    }

    fn resolve_local(&mut self, var_name: &str) -> Result<VariableType, CompilerError> {
        log::debug!("Resolve variable {}", var_name);

        for (i, local) in self.function.locals.iter().enumerate().rev() {
            if local.name.as_str() != var_name {
                continue;
            }

            if local.depth.is_none() {
                return Err(CompilerError::Unknown(format!(
                    "Can't read local variable in its own initializer: {}",
                    var_name
                )));
            };

            return Ok(VariableType::Local(i));
        }

        for local in self.function.inherited_locals.iter().rev() {
            if local.var_name.as_str() != var_name {
                continue;
            };
            return Ok(VariableType::Inherited(InheritedLocalPos::new(
                local.pos,
                local.depth,
            )));
        }

        for (i, global) in self.globals.as_vec().iter().enumerate() {
            match global {
                Global::Identifier(v) => {
                    if v.name.as_str() == var_name {
                        return Ok(VariableType::Global(i));
                    };
                }
                Global::Native(f) if f.name == var_name => {
                    return Ok(VariableType::Global(i));
                }
                _ => continue,
            }
        }

        return Err(CompilerError::ConstantNotFound(format!(
            "Unknown variable: {var_name}"
        )));
    }

    fn unary(&mut self, token: &ast::Unary) -> Result<(), CompilerError> {
        self.expression(&token.right)?;
        self.unary_operator(&token.operator);
        Ok(())
    }

    fn unary_operator(&mut self, token: &ast::UnaryOperator) {
        self.emit_op_code(match token {
            ast::UnaryOperator::Minus => OpCode::Negate,
            ast::UnaryOperator::Not => OpCode::Not,
        })
    }

    pub fn grouping(&mut self, token: &ast::Grouping) -> Result<(), CompilerError> {
        self.expression(&token.expr)
    }

    pub fn end(mut self) -> GlobalStore {
        match self.function.chunk.code.last() {
            Some(OpCode::Return(r)) => (),
            _ => {
                self.emit_op_code(OpCode::None);
                self.emit_op_code(OpCode::Return(Return::new(None)));
            }
        }
        self.globals
    }
}
