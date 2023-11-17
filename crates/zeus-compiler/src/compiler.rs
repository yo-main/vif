use zeus_scanner::Scanner;
use zeus_scanner::ScanningErrorType;
use zeus_scanner::Token;
use zeus_scanner::TokenType;

use crate::debug::disassemble_chunk;
use crate::error::CompilerError;
use crate::parser_rule::PrattParser;
use crate::precedence::Precedence;
use crate::Chunk;
use crate::Constant;
use crate::OpCode;

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    pending: Option<Token>,
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
                _ => {
                    log::error!("{}", e.msg);
                    Err(CompilerError::ScanningError(format!("{e}")))
                }
            },
        }
    }

    pub fn synchronize(&mut self) -> Result<(), CompilerError> {
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
            t if t.r#type == TokenType::Var => self.var_declaration(), // if we need to put it above, think about the pending var
            t if t.r#type == TokenType::NewLine => self.statement(),
            t => {
                self.pending = Some(t);
                self.expression_statement()
            }
        }
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

        match token.r#type {
            TokenType::Identifier(s) => Ok(self.register_constant(Constant::Identifier(s))),
            e => {
                return Err(CompilerError::SyntaxError(format!(
                    "Expected identifier when parsing variable, got {e}"
                )))
            }
        }
    }

    fn define_variable(&mut self, variable: usize) {
        self.emit_op_code(OpCode::OP_GLOBAL_VARIABLE(variable))
    }

    fn register_constant(&mut self, constant: Constant) -> usize {
        self.compiling_chunk.add_constant(constant)
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
                self.named_variable(Constant::Identifier(s.clone()), can_assign)
            }
            _ => return Err(CompilerError::Unknown(format!("Impossible"))),
        }
    }

    fn named_variable(
        &mut self,
        constant: Constant,
        can_assign: bool,
    ) -> Result<(), CompilerError> {
        let index = self.register_constant(constant);

        if can_assign && self.match_token(TokenType::Equal)? {
            self.expression()?;
            self.emit_op_code(OpCode::OP_SET_GLOBAL(index));
        } else {
            self.emit_op_code(OpCode::OP_GET_GLOBAL(index));
        }

        Ok(())
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
            TokenType::Integer(i) => self.emit_constant(Constant::Integer(*i)),
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
            TokenType::String(s) => self.emit_constant(Constant::String(s.clone())),
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

    fn emit_constant(&mut self, constant: Constant) {
        let index = self.compiling_chunk.add_constant(constant);
        self.emit_op_code(OpCode::OP_CONSTANT(index))
    }

    fn emit_op_code(&mut self, op_code: OpCode) {
        self.compiling_chunk
            .write_chunk(op_code, self.scanner.get_line())
    }
}
