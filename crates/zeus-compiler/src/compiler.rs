use zeus_scanner::Scanner;
use zeus_scanner::Token;
use zeus_scanner::TokenType;

use crate::error::CompilerError;
use crate::precedence::Precedence;
use crate::Chunk;
use crate::Constant;
use crate::OpCode;

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    tokens: Vec<Token>,
    pub compiling_chunk: Chunk,
}

// TODO: this mixes both parsing and translation into bytecode at the same time
// we should be able to split those 2 steps in distinct part: parsing AST, translation
// potentially we could add more steps in between (like optimization)
impl<'a> Compiler<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Compiler {
            scanner,
            tokens: Vec::new(),
            compiling_chunk: Chunk::new(),
        }
    }

    pub fn advance(&mut self) -> Result<(), CompilerError> {
        loop {
            match self.scanner.scan() {
                Ok(t) if t.r#type == TokenType::EOF => break,
                Ok(token) => self.tokens.push(token),
                Err(e) => {
                    log::error!("{}", e);
                    return Err(CompilerError::ScanningError(format!("{e}")));
                }
            };
        }
        Ok(())
    }
    pub fn expression(&mut self) -> Result<(), CompilerError> {
        self.parse_precedence(Precedence::Assignement)?;
        Ok(())
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), CompilerError> {
        match self.scanner.scan() {
            Ok(t) if t.r#type == token_type => Ok(()),
            Ok(_) => Err(CompilerError::SyntaxError(msg.to_owned())),
            Err(e) => Err(CompilerError::ScanningError(format!("{e}"))),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompilerError> {
        Ok(())
    }

    fn get_rule(&mut self, token_type: &TokenType) -> Result<Precedence, CompilerError> {
        Ok(Precedence::Assignement)
    }

    fn binary(&mut self, token_type: &TokenType) -> Result<(), CompilerError> {
        let rule = self.get_rule(token_type)?;
        self.parse_precedence(rule);

        match token_type {
            TokenType::Plus => self.emit_op_code(OpCode::OP_ADD),
            TokenType::Minus => self.emit_op_code(OpCode::OP_SUBSTRACT),
            TokenType::Star => self.emit_op_code(OpCode::OP_MULTIPLY),
            TokenType::Slash => self.emit_op_code(OpCode::OP_DIVIDE),
            _ => {
                return Err(CompilerError::Unknown(
                    "Expected an operator here".to_owned(),
                ))
            }
        };
        Ok(())
    }

    fn unary(&mut self, token_type: &TokenType) -> Result<(), CompilerError> {
        self.parse_precedence(Precedence::Unary);
        self.emit_op_code(OpCode::OP_NEGATE);
        Ok(())
    }

    fn grouping(&mut self) -> Result<(), CompilerError> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after an expression")
    }

    fn number(&mut self, number: &TokenType) -> Result<(), CompilerError> {
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

    pub fn end(&mut self) {
        self.emit_op_code(OpCode::OP_RETURN)
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
