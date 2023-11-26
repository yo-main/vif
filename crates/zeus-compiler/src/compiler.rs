use zeus_scanner::Scanner;
use zeus_scanner::ScanningErrorType;
use zeus_scanner::Token;
use zeus_scanner::TokenType;

use crate::debug::disassemble_chunk;
use crate::error::CompilerError;
use crate::function::Function;
use crate::local::Local;
use crate::parser_rule::PrattParser;
use crate::precedence::Precedence;
use crate::OpCode;
use crate::Variable;

pub struct Compiler<'scanner, 'function, 'a> {
    scanner: &'scanner mut Scanner<'a>,
    pending: Option<Token>,
    scope_depth: usize,
    loop_details: Vec<(usize, usize)>,
    globals: Vec<Variable>,
    function: &'function mut Function,
}

// TODO: this mixes both parsing and translation into bytecode at the same time
// we should be able to split those 2 steps in distinct part: parsing AST, translation
// potentially we could add more steps in between (like optimization)
impl<'scanner, 'function, 'a> Compiler<'scanner, 'function, 'a> {
    pub fn new(scanner: &'scanner mut Scanner<'a>, function: &'function mut Function) -> Self {
        Compiler {
            scanner,
            function,
            pending: None,
            scope_depth: 0,
            loop_details: Vec::new(),
            globals: Vec::new(),
        }
    }

    fn emit_op_code(&mut self, op_code: OpCode) {
        self.function
            .chunk
            .write_chunk(op_code, self.scanner.get_line());
    }
    fn emit_jump(&mut self, op_code: OpCode) -> usize {
        self.emit_op_code(op_code);
        self.function.chunk.code.len() - 1
    }

    fn emit_constant(&mut self, variable: Variable) {
        let index = self.globals.push(variable);
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

    pub fn advance(&mut self) -> Result<Token, CompilerError> {
        if self.pending.is_some() {
            return Ok(self.pending.take().unwrap());
        };

        match self.scanner.scan() {
            Ok(token) => match token.r#type {
                TokenType::Ignore => self.advance(),
                _ => Ok(token),
            },
            Err(e) => match e.r#type {
                ScanningErrorType::EOF => Err(CompilerError::EOF),
                _ => Err(CompilerError::ScanningError(format!("{e}"))),
            },
        }
    }

    pub fn synchronize(&mut self) -> Result<(), CompilerError> {
        log::debug!("Resynchronizing compiler");
        if self.scanner.is_at_line_start() {
            return Ok(());
        }

        loop {
            match self.advance()? {
                t if t.r#type == TokenType::NewLine => return Ok(()),
                t if t.r#type == TokenType::Class => return Ok(()),
                t if t.r#type == TokenType::Def => return Ok(()),
                t if t.r#type == TokenType::Var => return Ok(()),
                t if t.r#type == TokenType::For => return Ok(()),
                t if t.r#type == TokenType::If => return Ok(()),
                t if t.r#type == TokenType::While => return Ok(()),
                t if t.r#type == TokenType::Return => return Ok(()),
                _ => (),
            }
        }
    }

    pub fn declaration(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting declaration");
        match self.advance()? {
            t if t.r#type == TokenType::Def => self.function_declaration(),
            t if t.r#type == TokenType::Var => self.var_declaration(),
            t => {
                self.pending = Some(t);
                self.statement()
            }
        }
    }

    pub fn statement(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting statement");
        match self.advance()? {
            t if t.r#type == TokenType::At => {
                self.expression()?;
                self.consume(TokenType::NewLine, "Expects new line after expression")?;
                self.emit_op_code(OpCode::Print);
                return Ok(());
            }
            t if t.r#type == TokenType::If => self.if_statement(),
            t if t.r#type == TokenType::NewLine => self.statement(),
            t if t.r#type == TokenType::While => self.while_statement(),
            t if t.r#type == TokenType::Indent => {
                self.begin_scope();
                let res = self.block();
                self.end_scope();
                return res;
            }
            t => {
                self.pending = Some(t);
                self.expression_statement()
            }
        }
    }

    fn if_statement(&mut self) -> Result<(), CompilerError> {
        self.expression()?;
        self.consume(TokenType::DoubleDot, "Expects : after if statement")?;
        self.consume(TokenType::NewLine, "Expects \\n after if statement")?;

        let then_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        self.emit_op_code(OpCode::Pop);
        self.statement()?;

        let else_jump = self.emit_jump(OpCode::Jump(self.function.chunk.code.len()));
        self.patch_jump(then_jump);
        self.emit_op_code(OpCode::Pop);

        let res = match self.advance() {
            Ok(t) if t.r#type == TokenType::Else => {
                self.consume(TokenType::DoubleDot, "Expects : after else statement")?;
                self.consume(TokenType::NewLine, "Expects \\n after else statement")?;

                self.statement()
            }
            Ok(t) if t.r#type == TokenType::ElIf => self.if_statement(),
            Ok(t) => {
                self.pending = Some(t);
                Ok(())
            }
            Err(e) => Err(e),
        };

        self.patch_jump(else_jump);

        res
    }

    fn while_statement(&mut self) -> Result<(), CompilerError> {
        let loop_start = self.function.chunk.code.len();

        self.expression()?;
        self.consume(TokenType::DoubleDot, "Expects : after else statement")?;
        self.consume(TokenType::NewLine, "Expects \\n after else statement")?;

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        self.loop_details.push((loop_start, exit_jump));
        self.emit_op_code(OpCode::Pop);
        let res = self.statement();
        self.loop_details.pop().unwrap();
        self.emit_op_code(OpCode::Goto(loop_start));

        self.patch_jump(exit_jump);
        self.emit_op_code(OpCode::Pop);

        res
    }

    pub fn break_loop(&mut self) -> Result<(), CompilerError> {
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

    fn function_declaration(&mut self) -> Result<(), CompilerError> {
        let var = self.parse_variable()?;
        self.function_statement()?;
        self.define_variable(var);
        Ok(())
    }

    fn function_statement(&mut self) -> Result<(), CompilerError> {
        let mut function = Function::new(0, "function".to_owned());
        let mut compiler = Compiler::new(self.scanner, &mut function);

        compiler.begin_scope();
        compiler.consume(TokenType::LeftParen, "Expects ( after function name")?;
        match compiler.advance()? {
            mut t if t.r#type == TokenType::Comma => {
                while t.r#type == TokenType::Comma {
                    compiler.function.arity += 1;
                    let variable = compiler.parse_variable()?;
                    compiler.define_variable(variable);
                    t = compiler.advance()?;
                }
            }
            t if t.r#type == TokenType::RightParen => (),
            t => {
                return Err(CompilerError::SyntaxError(format!(
                    "Unexpected char in function declaration: {t}"
                )))
            }
        }
        compiler.consume(TokenType::DoubleDot, "Expects : after function declaration")?;
        compiler.block()?;
        compiler.end();
        self.emit_constant(Variable::Function(function));

        Ok(())
    }

    fn block(&mut self) -> Result<(), CompilerError> {
        loop {
            match self.advance()? {
                t if t.r#type == TokenType::Dedent => break,
                t if t.r#type == TokenType::NewLine => (),
                t => {
                    self.pending = Some(t);
                    self.declaration()?;
                }
            }
        }

        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1
    }

    fn end_scope(&mut self) {
        while let Some(variable) = self.function.locals.last() {
            // TODO: maybe use a match here ?
            if variable.depth.unwrap_or(usize::MAX) >= self.scope_depth {
                self.function.locals.pop().unwrap();
                self.emit_op_code(OpCode::Pop);
            }
        }

        self.scope_depth -= 1
    }

    fn var_declaration(&mut self) -> Result<(), CompilerError> {
        let variable = self.parse_variable()?;

        match self.advance()? {
            t if t.r#type == TokenType::Equal => self.expression()?,
            _ => {
                return Err(CompilerError::SyntaxError(format!(
                    "Expected an assignement after var declaration"
                )))
            }
        };

        self.consume(
            TokenType::NewLine,
            "Expects new line after variable declaration",
        )?;

        self.define_variable(variable);
        Ok(())
    }

    fn parse_variable(&mut self) -> Result<usize, CompilerError> {
        let token = self.advance()?;

        let variable = match token.r#type {
            TokenType::Identifier(s) => Variable::Identifier(s),
            e => {
                return Err(CompilerError::SyntaxError(format!(
                    "Expected identifier when parsing variable, got {e}"
                )))
            }
        };
        Ok(self.register_variable(variable))
    }

    fn initialize_variable(&mut self) {
        if let Some(var) = self.function.locals.last_mut() {
            var.depth = Some(self.scope_depth);
        }
    }

    fn define_variable(&mut self, variable: usize) {
        if self.scope_depth > 0 {
            self.initialize_variable();
        } else {
            self.emit_op_code(OpCode::GlobalVariable(variable))
        }
    }

    fn register_variable(&mut self, variable: Variable) -> usize {
        if self.scope_depth > 0 {
            self.function.locals.push(Local::new(variable, None));
            0
        } else {
            self.globals.push(variable);
            self.globals.len() - 1
        }
    }

    fn expression_statement(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting expression statement");
        self.expression()?;
        self.consume(TokenType::NewLine, "Expects \\n after an expression")?;
        self.emit_op_code(OpCode::Pop);
        Ok(())
    }

    fn expression(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting expression");
        self.parse_precedence(Precedence::Assignement)?;
        Ok(())
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), CompilerError> {
        match self.advance() {
            Ok(t) if t.r#type == token_type => Ok(()),
            Ok(t) => Err(CompilerError::SyntaxError(format!(
                "{}, got {}",
                msg.to_owned(),
                t
            ))),
            Err(e) => Err(CompilerError::ScanningError(format!("{e}"))),
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> Result<bool, CompilerError> {
        Ok(match self.advance()? {
            t if t.r#type == token_type => true,
            t => {
                self.pending = Some(t);
                false
            }
        })
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompilerError> {
        let precedence = precedence as u8;

        let token = self.advance()?;
        let can_assign = precedence <= Precedence::Assignement as u8;

        token.r#type.prefix(self, can_assign)?;
        log::debug!("parse precedence in with {}", precedence);

        loop {
            let token = self.advance()?;
            if precedence > token.r#type.precedence() as u8 {
                self.pending = Some(token);
                break;
            }
            token.r#type.infix(self)?;
        }
        if can_assign && self.match_token(TokenType::Equal)? {
            return Err(CompilerError::SyntaxError(format!(
                "Invalid assignment target"
            )));
        }

        log::debug!("parse precedence out with {}", token.r#type.precedence());

        Ok(())
    }

    fn get_rule(&mut self, token_type: &TokenType) -> Precedence {
        token_type.precedence()
    }

    pub fn and(&mut self) -> Result<(), CompilerError> {
        let end_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        self.emit_op_code(OpCode::Pop);
        let res = self.parse_precedence(Precedence::And);
        self.patch_jump(end_jump);
        res
    }

    pub fn or(&mut self) -> Result<(), CompilerError> {
        let else_jump = self.emit_jump(OpCode::JumpIfFalse(self.function.chunk.code.len()));
        let end_jump = self.emit_jump(OpCode::Jump(self.function.chunk.code.len()));

        self.patch_jump(else_jump);
        self.emit_op_code(OpCode::Pop);

        let res = self.parse_precedence(Precedence::Or);
        self.patch_jump(end_jump);
        res
    }

    pub fn variable(
        &mut self,
        token_type: &TokenType,
        can_assign: bool,
    ) -> Result<(), CompilerError> {
        match token_type {
            TokenType::Identifier(s) => {
                self.named_variable(Variable::Identifier(s.clone()), can_assign)
            }
            _ => return Err(CompilerError::Unknown(format!("Impossible"))),
        }
    }

    fn named_variable(
        &mut self,
        variable: Variable,
        can_assign: bool,
    ) -> Result<(), CompilerError> {
        let is_set = can_assign && self.match_token(TokenType::Equal)?;

        let op_code = match self.resolve_local(&variable)? {
            Some(index) => match is_set {
                true => {
                    self.expression()?;
                    OpCode::SetLocal(index)
                }
                false => OpCode::GetLocal(index),
            },
            None => {
                let index = self.register_variable(variable);
                match is_set {
                    true => {
                        self.expression()?;
                        OpCode::SetGlobal(index)
                    }
                    false => OpCode::GetGlobal(index),
                }
            }
        };

        self.emit_op_code(op_code);

        Ok(())
    }

    fn resolve_local(&mut self, variable: &Variable) -> Result<Option<usize>, CompilerError> {
        let var_name = match variable {
            Variable::Identifier(s) => s,
            _ => return Ok(None), // TODO: I beg you to change that
        };

        for (i, local) in self.function.locals.iter().rev().enumerate() {
            match &local.variable {
                Variable::Identifier(s) if s == var_name => match local.depth {
                    None => {
                        return Err(CompilerError::Unknown(format!(
                            "Can't read local variable in its own initializer"
                        )))
                    }
                    Some(_) => return Ok(Some(self.function.locals.len() - i - 1)),
                },
                _ => (),
            }
        }

        return Ok(None);
    }

    pub fn binary(&mut self, token_type: &TokenType) -> Result<(), CompilerError> {
        let rule = self.get_rule(token_type);
        self.parse_precedence(rule)?;

        match token_type {
            TokenType::Plus => self.emit_op_code(OpCode::Add),
            TokenType::Minus => self.emit_op_code(OpCode::Substract),
            TokenType::Star => self.emit_op_code(OpCode::Multiply),
            TokenType::Slash => self.emit_op_code(OpCode::Divide),
            TokenType::EqualEqual => self.emit_op_code(OpCode::Equal),
            TokenType::BangEqual => self.emit_op_code(OpCode::NotEqual),
            TokenType::Greater => self.emit_op_code(OpCode::Greater),
            TokenType::GreaterEqual => self.emit_op_code(OpCode::GreaterOrEqual),
            TokenType::Less => self.emit_op_code(OpCode::Less),
            TokenType::LessEqual => self.emit_op_code(OpCode::LessOrEqual),
            e => {
                return Err(CompilerError::Unknown(format!(
                    "Expected an operator here, got {e}"
                )))
            }
        };
        Ok(())
    }

    pub fn unary(&mut self, token: &TokenType) -> Result<(), CompilerError> {
        log::debug!("Unary starting");
        self.parse_precedence(Precedence::Unary)?;
        match token {
            TokenType::Minus => self.emit_op_code(OpCode::Negate),
            TokenType::Not => self.emit_op_code(OpCode::Not),
            e => {
                return Err(CompilerError::Unknown(format!(
                    "Expected a unary operator here, got {e}"
                )))
            }
        }
        Ok(())
    }

    pub fn grouping(&mut self) -> Result<(), CompilerError> {
        log::debug!("Grouping starting");
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after an expression")
    }

    pub fn number(&mut self, number: &TokenType) -> Result<(), CompilerError> {
        log::debug!("Number starting");
        match number {
            TokenType::Integer(i) => self.emit_constant(Variable::Integer(*i)),
            _ => {
                return Err(CompilerError::Unknown(
                    "Should not have been something else than number".to_owned(),
                ))
            }
        }

        Ok(())
    }

    pub fn string(&mut self, token: &TokenType) -> Result<(), CompilerError> {
        log::debug!("String starting");
        match token {
            TokenType::String(s) => self.emit_constant(Variable::String(s.clone())),
            _ => {
                return Err(CompilerError::Unknown(
                    "Should not have been something else than number".to_owned(),
                ))
            }
        }

        Ok(())
    }

    pub fn literal(&mut self, token: &TokenType) -> Result<(), CompilerError> {
        match token {
            TokenType::False => self.emit_op_code(OpCode::False),
            TokenType::True => self.emit_op_code(OpCode::True),
            TokenType::None => self.emit_op_code(OpCode::None),
            e => {
                return Err(CompilerError::Unknown(format!(
                    "Expected a literal, got {e}"
                )))
            }
        };

        Ok(())
    }

    pub fn end(mut self) -> Vec<Variable> {
        self.emit_op_code(OpCode::Return);
        disassemble_chunk(
            &self.function.chunk,
            self.function.name.as_str(),
            &self.globals,
        );
        self.globals
    }
}
