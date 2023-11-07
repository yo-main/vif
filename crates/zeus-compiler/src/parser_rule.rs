use crate::compiler::Compiler;
use crate::error::CompilerError;
use crate::precedence::Precedence;
use zeus_scanner::TokenType;

pub trait PrattParser {
    fn infix(&self, compiler: &mut Compiler) -> Result<(), CompilerError>;
    fn prefix(&self, compiler: &mut Compiler) -> Result<(), CompilerError>;
    fn precedence(&self) -> Precedence;
}

impl PrattParser for TokenType {
    fn infix(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        match self {
            Self::LeftParen => compiler.grouping(),
            Self::Minus => compiler.unary(self),
            Self::Integer(_) => compiler.number(self),
            _ => Ok(()),
        }
    }

    fn prefix(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        match self {
            Self::Minus => compiler.binary(self),
            Self::Plus => compiler.binary(self),
            Self::Slash => compiler.binary(self),
            Self::Star => compiler.binary(self),
            _ => Ok(()),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            TokenType::Minus => Precedence::Term,
            TokenType::Plus => Precedence::Term,
            TokenType::Slash => Precedence::Factor,
            TokenType::Star => Precedence::Factor,
            _ => Precedence::None,
        }
    }
}
