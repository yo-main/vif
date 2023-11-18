use zeus_scanner::Scanner;
use zeus_scanner::ScanningErrorType;
use zeus_scanner::Token;
use zeus_scanner::TokenType;

use crate::debug::disassemble_chunk;
use crate::error::CompilerError;
use crate::local::Local;
use crate::parser_rule::PrattParser;
use crate::precedence::Precedence;
use crate::Chunk;
use crate::OpCode;
use crate::Variable;

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    pending: Option<Token>,
    locals: Vec<Local>,
    scope_depth: usize,
    pub compiling_chunk: Chunk,
}

// TODO: this mixes both parsing and translation into bytecode at the same time
// we should be able to split those 2 steps in distinct part: parsing AST, translation
// potentially we could add more steps in between (like optimization)
impl<'a> Compiler<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Compiler {
            scanner,
            pending: None,
            locals: Vec::new(),
            scope_depth: 0,
            compiling_chunk: Chunk::new(),
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
        self.statement()
    }

    fn statement(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting statement");
        match self.advance()? {
            t if t.r#type == TokenType::At => {
                self.expression()?;
                self.consume(TokenType::NewLine, "Expects new line after expression")?;
                self.emit_op_code(OpCode::OP_PRINT);
                return Ok(());
            }
            t if t.r#type == TokenType::If => self.if_statement(),
            t if t.r#type == TokenType::Var => self.var_declaration(), // if we need to put it above, think about the pending var
            t if t.r#type == TokenType::NewLine => self.statement(),
            t if t.r#type == TokenType::Indent => {
                self.begin_scope();
                self.block()?;
                self.end_scope();
                return Ok(());
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

        let then_jump = self.emit_jump(OpCode::OP_JUMP_IF_FALSE(self.compiling_chunk.code.len()));
        self.emit_op_code(OpCode::OP_POP);
        self.statement()?;

        self.patch_jump(then_jump);
        // the below code is supposed to remove implicit else clause but I think I don't have that
        // self.emit_op_code(OpCode::OP_POP);

        if self.match_token(TokenType::Else)? {
            let else_jump = self.emit_jump(OpCode::OP_JUMP(self.compiling_chunk.code.len()));
            self.statement()?;
            self.patch_jump(else_jump);
        }

        Ok(())
    }

    fn emit_jump(&mut self, op_code: OpCode) -> usize {
        self.emit_op_code(op_code);
        self.compiling_chunk.code.len() - 1
    }

    fn patch_jump(&mut self, offset: usize) {
        let curr = self.compiling_chunk.code.len();
        match self.compiling_chunk.code.get_mut(offset) {
            Some(OpCode::OP_JUMP_IF_FALSE(ref mut i)) => *i = curr - *i - 1,
            Some(OpCode::OP_JUMP(ref mut i)) => *i = curr - *i - 1,
            _ => (),
        }
    }

    fn block(&mut self) -> Result<(), CompilerError> {
        loop {
            match self.advance()? {
                t if t.r#type == TokenType::Dedent => break,
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
        while let Some(variable) = self.locals.last() {
            if variable.depth.unwrap_or(usize::MAX) >= self.scope_depth {
                self.locals.pop().unwrap();
                self.emit_op_code(OpCode::OP_POP);
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

        match self.scope_depth {
            0 => Ok(self.register_constant(variable)),
            _ => {
                self.declare_variable(variable);
                return Ok(0);
            }
        }
    }

    fn declare_variable(&mut self, variable: Variable) {
        self.add_local(variable);
    }

    fn add_local(&mut self, variable: Variable) {
        self.locals.push(Local::new(variable, None))
    }

    fn define_variable(&mut self, variable: usize) {
        if self.scope_depth > 0 {
            if let Some(var) = self.locals.last_mut() {
                var.depth = Some(self.scope_depth);
            }
            return;
        }
        self.emit_op_code(OpCode::OP_GLOBAL_VARIABLE(variable))
    }

    fn register_constant(&mut self, variable: Variable) -> usize {
        self.compiling_chunk.add_constant(variable)
    }

    fn expression_statement(&mut self) -> Result<(), CompilerError> {
        log::debug!("Starting expression statement");
        self.expression()?;
        self.consume(TokenType::NewLine, "Expects \\n after an expression")?;
        self.emit_op_code(OpCode::OP_POP);
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
                    OpCode::OP_SET_LOCAL(index)
                }
                false => OpCode::OP_GET_LOCAL(index),
            },
            None => {
                let index = self.register_constant(variable);
                match is_set {
                    true => {
                        self.expression()?;
                        OpCode::OP_SET_GLOBAL(index)
                    }
                    false => OpCode::OP_GET_GLOBAL(index),
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

        for (i, local) in self.locals.iter().rev().enumerate() {
            match &local.variable {
                Variable::Identifier(s) if s == var_name => match local.depth {
                    None => {
                        return Err(CompilerError::Unknown(format!(
                            "Can't read local variable in its own initializer"
                        )))
                    }
                    Some(_) => return Ok(Some(self.locals.len() - i - 1)),
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
            TokenType::Plus => self.emit_op_code(OpCode::OP_ADD),
            TokenType::Minus => self.emit_op_code(OpCode::OP_SUBSTRACT),
            TokenType::Star => self.emit_op_code(OpCode::OP_MULTIPLY),
            TokenType::Slash => self.emit_op_code(OpCode::OP_DIVIDE),
            TokenType::EqualEqual => self.emit_op_code(OpCode::OP_EQUAL),
            TokenType::BangEqual => self.emit_op_code(OpCode::OP_NOT_EQUAL),
            TokenType::Greater => self.emit_op_code(OpCode::OP_GREATER),
            TokenType::GreaterEqual => self.emit_op_code(OpCode::OP_GREATER_OR_EQUAL),
            TokenType::Less => self.emit_op_code(OpCode::OP_LESS),
            TokenType::LessEqual => self.emit_op_code(OpCode::OP_LESS_OR_EQUAL),
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
            TokenType::Minus => self.emit_op_code(OpCode::OP_NEGATE),
            TokenType::Not => self.emit_op_code(OpCode::OP_NOT),
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
            TokenType::False => self.emit_op_code(OpCode::OP_FALSE),
            TokenType::True => self.emit_op_code(OpCode::OP_TRUE),
            TokenType::None => self.emit_op_code(OpCode::OP_NONE),
            e => {
                return Err(CompilerError::Unknown(format!(
                    "Expected a literal, got {e}"
                )))
            }
        };

        Ok(())
    }

    pub fn end(&mut self) {
        self.emit_op_code(OpCode::OP_RETURN);
        disassemble_chunk(&self.compiling_chunk, "code");
    }

    fn emit_constant(&mut self, variable: Variable) {
        let index = self.compiling_chunk.add_constant(variable);
        self.emit_op_code(OpCode::OP_CONSTANT(index))
    }

    fn emit_op_code(&mut self, op_code: OpCode) {
        self.compiling_chunk
            .write_chunk(op_code, self.scanner.get_line())
    }
}
