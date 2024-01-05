use crate::debug::disassemble_chunk;
use crate::error::CompilerError;
use crate::NativeFunction;
use crate::NativeFunctionCallee;
use crate::OpCode;
use crate::Variable;
use zeus_objects::ast;
use zeus_objects::function::Arity;
use zeus_objects::function::Function;
use zeus_objects::global::Global;
use zeus_objects::local::InheritedLocalPos;
use zeus_objects::local::Local;
use zeus_objects::local::VariableType;

pub struct Compiler<'function> {
    scope_depth: usize,
    loop_details: Vec<(usize, usize)>,
    globals: Global,
    function: &'function mut Function,
}

impl<'function> Compiler<'function> {
    pub fn new(function: &'function mut Function, scope_depth: usize) -> Self {
        Compiler {
            function,
            scope_depth,
            loop_details: Vec::new(),
            globals: Global::new(),
        }
    }

    pub fn compile(&mut self, ast: &Vec<ast::Stmt>) -> Result<(), CompilerError> {
        for token in ast.iter() {
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

    fn emit_constant(&mut self, variable: Variable) {
        self.globals.push(variable);
        self.emit_op_code(OpCode::Constant(self.globals.len() - 1))
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
        self.emit_op_code(OpCode::Return);
        Ok(())
    }

    pub fn call(&mut self, token: &ast::Call) -> Result<(), CompilerError> {
        log::debug!("Starting call");
        self.expression(&token.callee)?;

        for arg in token.arguments.iter() {
            self.expression(arg)?;
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
        let index = self.register_variable(Variable::Identifier(Box::new(token.name.clone())));
        self.function_statement(token)?;
        self.define_variable(index);
        Ok(())
    }

    fn function_statement(&mut self, token: &ast::Function) -> Result<(), CompilerError> {
        log::debug!("Starting function statement");
        let mut function = Function::new(Arity::Fixed(token.params.len()), token.name.clone());

        if self.scope_depth > 0 {
            // TODO: manage self.function.inherited_locals as well

            for (i, local) in self.function.locals.iter().enumerate() {
                match &local.variable {
                    Variable::Identifier(s) => {
                        function
                            .inherited_locals
                            .push(zeus_objects::local::InheritedLocal {
                                var_name: s.clone(),
                                depth: self.scope_depth,
                                pos: i + 1,
                            })
                    }
                    _ => (),
                }
            }
        }

        let mut compiler = Compiler::new(&mut function, self.scope_depth + 1);
        std::mem::swap(&mut compiler.globals, &mut self.globals);

        for variable in token.params.iter() {
            compiler.register_variable(Variable::Identifier(Box::new(variable.clone())));
            compiler.initialize_variable();
        }

        log::debug!("Function compiling starting");
        compiler.compile(&token.body)?;

        // compiler.end_scope();
        let mut globals = compiler.end();
        std::mem::swap(&mut globals, &mut self.globals);
        log::debug!("Function compiling terminated");
        self.emit_constant(Variable::Function(Box::new(function)));
        Ok(())
    }

    fn block(&mut self, token: &Vec<ast::Stmt>) -> Result<(), CompilerError> {
        log::debug!("Block starting");
        for block in token.iter() {
            self.statement(block)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1
    }

    // fn end_scope(&mut self) {
    //     // Since we adjust stack on the VM, I'm wondering if that part is still needed ?
    //     while let Some(variable) = self.function.locals.last() {
    //         // TODO: maybe use a match here ?
    //         if variable.depth.unwrap_or(usize::MAX) >= self.scope_depth {
    //             self.function.locals.pop().unwrap();
    //             self.emit_op_code(OpCode::Pop);
    //         }
    //     }

    //     self.scope_depth -= 1
    // }

    fn var_declaration(&mut self, token: &ast::Variable) -> Result<(), CompilerError> {
        log::debug!("Starting variable declaration");
        let index = self.register_variable(Variable::Identifier(Box::new(token.name.clone())));
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
        if self.scope_depth > 0 {
            self.initialize_variable();
            return;
        }

        self.emit_op_code(OpCode::GlobalVariable(variable_index));
    }

    fn register_variable(&mut self, variable: Variable) -> usize {
        log::debug!("Register variable {}", variable);
        if self.scope_depth > 0 {
            self.function.locals.push(Local::new(variable, None));
            0
        } else {
            self.make_constant(variable)
        }
    }

    fn make_constant(&mut self, variable: Variable) -> usize {
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
        match token.as_ref() {
            ast::Expr::Binary(t) => self.binary(t),
            ast::Expr::Unary(t) => self.unary(t),
            ast::Expr::Grouping(t) => self.grouping(t),
            ast::Expr::Literal(t) => self.literal(t),
            ast::Expr::Value(t) => self.value(t),
            ast::Expr::Assign(t) => self.assign(t),
            ast::Expr::Logical(t) => self.logical(t),
            ast::Expr::Call(t) => self.call(t),
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
        self.variable(token.name.as_str(), true)
    }

    fn value(&mut self, token: &ast::Value) -> Result<(), CompilerError> {
        match token {
            ast::Value::Operator(o) => self.operator(o),
            ast::Value::String(s) => self.emit_constant(Variable::String(Box::new(s.clone()))),
            ast::Value::Integer(i) => self.emit_constant(Variable::Integer(*i)),
            ast::Value::Float(f) => self.emit_constant(Variable::Float(*f)),
            ast::Value::Variable(s) => self.variable(s, false)?,
            ast::Value::True => self.emit_op_code(OpCode::True),
            ast::Value::False => self.emit_op_code(OpCode::False),
            ast::Value::Break => self.break_loop()?,
            ast::Value::Continue => self.continue_loop()?,
            ast::Value::NewLine => (),
            ast::Value::None => self.emit_op_code(OpCode::None),
            ast::Value::Ignore => (),
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

    pub fn variable(&mut self, var_name: &str, assign: bool) -> Result<(), CompilerError> {
        log::debug!("Starting variable");
        match var_name {
            "get_time" => self.named_variable(
                Variable::Native(NativeFunction::new(NativeFunctionCallee::GetTime)),
                assign,
            ),
            "print" => self.named_variable(
                Variable::Native(NativeFunction::new(NativeFunctionCallee::Print)),
                assign,
            ),
            _ => self.named_variable(Variable::Identifier(Box::new(var_name.to_owned())), assign),
        }
    }

    fn named_variable(&mut self, variable: Variable, assign: bool) -> Result<(), CompilerError> {
        log::debug!("Named variable {} (assign={})", variable, assign);

        let op_code = match self.resolve_local(&variable)? {
            VariableType::Local(index) => match assign {
                true => OpCode::SetLocal(index),
                false => OpCode::GetLocal(index),
            },
            VariableType::Inherited(v) => match assign {
                true => OpCode::SetInheritedLocal(v),
                false => OpCode::GetInheritedLocal(v),
            },
            VariableType::None => {
                let index = self.make_constant(variable);
                match assign {
                    true => OpCode::SetGlobal(index),
                    false => OpCode::GetGlobal(index),
                }
            }
        };

        self.emit_op_code(op_code);
        Ok(())
    }

    fn resolve_local(&mut self, variable: &Variable) -> Result<VariableType, CompilerError> {
        log::debug!("Resolve variable {}", variable);
        let var_name = match variable {
            Variable::Identifier(s) => s,
            Variable::Native(f) => f.name,
            e => return Err(CompilerError::Unknown(format!("Trying to resolve {e}"))),
        };

        for (i, local) in self.function.locals.iter().rev().enumerate() {
            match &local.variable {
                Variable::Identifier(s) if s.as_str() == var_name => match local.depth {
                    None => {
                        return Err(CompilerError::Unknown(format!(
                            "Can't read local variable in its own initializer"
                        )));
                    }
                    Some(_) => return Ok(VariableType::Local(self.function.locals.len() - i)),
                },
                _ => (),
            }
        }

        for local in self.function.inherited_locals.iter().rev() {
            if local.var_name.as_str() == var_name {
                return Ok(VariableType::Inherited(InheritedLocalPos {
                    pos: local.pos,
                    depth: local.depth,
                }));
            }
        }

        return Ok(VariableType::None);
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

    pub fn literal(&mut self, token: &ast::Literal) -> Result<(), CompilerError> {
        match token {
            ast::Literal::String(s) => self.emit_constant(Variable::String(Box::new(s.clone()))),
            ast::Literal::Identifier(i) => {
                self.emit_constant(Variable::Identifier(Box::new(i.clone())))
            }
        }
        Ok(())
    }

    pub fn end(mut self) -> Global {
        match self.function.chunk.code.last() {
            Some(OpCode::Return) => (),
            _ => {
                self.emit_op_code(OpCode::None);
                self.emit_op_code(OpCode::Return);
            }
        }
        disassemble_chunk(
            &self.function.chunk,
            self.function.name.as_str(),
            &self.globals,
        );
        self.globals
    }
}
