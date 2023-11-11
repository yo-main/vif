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
    pub fn expression(&mut self) -> Result<(), CompilerError> {
        self.parse_precedence(Precedence::Assignement)?;
        Ok(())
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), CompilerError> {
        match self.advance() {
            Ok(t) if t.r#type == token_type => Ok(()),
            Ok(_) => Err(CompilerError::SyntaxError(msg.to_owned())),
            Err(e) => Err(CompilerError::ScanningError(format!("{e}"))),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompilerError> {
        let token = self.advance()?;
        token.r#type.prefix(self)?;
        log::debug!("parse precedence in with {}", precedence);

        let precedence = precedence as u8;

        loop {
            let token = self.advance()?;
            if precedence > token.r#type.precedence() as u8 {
                self.pending = Some(token);
                break;
            }
            token.r#type.infix(self)?;
        }
        log::debug!("parse precedence out with {}", token.r#type.precedence());

        Ok(())
    }

    fn get_rule(&mut self, token_type: &TokenType) -> Precedence {
        token_type.precedence()
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
