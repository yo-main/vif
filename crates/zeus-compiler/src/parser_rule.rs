use crate::compiler::Compiler;
use crate::error::CompilerError;
use crate::precedence::Precedence;
use zeus_scanner::TokenType;

pub trait PrattParser {
    fn infix(&self, compiler: &mut Compiler) -> Result<(), CompilerError>;
    fn prefix(&self, compiler: &mut Compiler, can_assign: bool) -> Result<(), CompilerError>;
    fn precedence(&self) -> Precedence;
}

impl PrattParser for TokenType {
    fn prefix(&self, compiler: &mut Compiler, can_assign: bool) -> Result<(), CompilerError> {
        log::debug!("Prefix: {}", self);
        match self {
            Self::LeftParen => compiler.grouping(),
            Self::Minus => compiler.unary(self),
            Self::Integer(_) => compiler.number(self),
            Self::False => compiler.literal(self),
            Self::True => compiler.literal(self),
            Self::None => compiler.literal(self),
            Self::Not => compiler.unary(self),
            Self::String(_) => compiler.string(self),
            Self::Identifier(_) => compiler.variable(self, can_assign),
            _ => Ok(()),
        }
    }

    fn infix(&self, compiler: &mut Compiler) -> Result<(), CompilerError> {
        log::debug!("Infix: {}", self);
        match self {
            Self::Minus => compiler.binary(self),
            Self::Plus => compiler.binary(self),
            Self::Slash => compiler.binary(self),
            Self::Star => compiler.binary(self),
            Self::BangEqual => compiler.binary(self),
            Self::EqualEqual => compiler.binary(self),
            Self::Greater => compiler.binary(self),
            Self::GreaterEqual => compiler.binary(self),
            Self::Less => compiler.binary(self),
            Self::LessEqual => compiler.binary(self),
            Self::And => compiler.and(),
            Self::Or => compiler.or(),
            _ => Ok(()),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Minus => Precedence::Term,
            Self::Plus => Precedence::Term,
            Self::Slash => Precedence::Factor,
            Self::Star => Precedence::Factor,
            Self::BangEqual => Precedence::Equality,
            Self::EqualEqual => Precedence::Equality,
            Self::Greater => Precedence::Comparison,
            Self::GreaterEqual => Precedence::Comparison,
            Self::Less => Precedence::Comparison,
            Self::LessEqual => Precedence::Comparison,
            Self::And => Precedence::And,
            Self::Or => Precedence::Or,
            _ => Precedence::None,
        }
    }
}