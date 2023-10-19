use clap::error::Result;

use crate::ast::{Binary, Expr, Grouping, Literal, Unary, Value};
use crate::errors::ZeusErrorType;
use crate::tokens::Token;
use crate::tokens::TokenType;
use std::iter::Iterator;
use std::iter::Peekable;

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    pub ast: Vec<Box<Expr>>,
    pub errors: Vec<ZeusErrorType>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            ast: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> bool {
        loop {
            match self.expression() {
                Ok(expr) => self.ast.push(expr),
                Err(ZeusErrorType::EOF) => break,
                Err(err) => self.errors.push(err),
            }
        }

        self.errors.is_empty()
    }

    pub fn parse_error(&mut self) -> Result<(), ZeusErrorType> {
        let prev = self.advance().unwrap(); // we know that one if wrong
        if prev.r#type == TokenType::NewLine {
            return Ok(());
        }
        loop {
            match self.peek() {
                Some(t) if t.r#type == TokenType::Class => return Ok(()),
                Some(t) if t.r#type == TokenType::Def => return Ok(()),
                Some(t) if t.r#type == TokenType::For => return Ok(()),
                Some(t) if t.r#type == TokenType::While => return Ok(()),
                Some(t) if t.r#type == TokenType::Return => return Ok(()),
                Some(t) if t.r#type == TokenType::Let => return Ok(()),
                Some(t) if t.r#type == TokenType::Const => return Ok(()),
                _ => self.advance()?,
            };
        }
    }

    fn expression(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.equality()?;

        if self.r#match(&TokenType::Comma) {
            let operator = self.advance().unwrap();
            let right = self.expression()?;
            return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.comparison()?;

        for token_type in [&TokenType::BangEqual, &TokenType::EqualEqual] {
            if self.r#match(token_type) {
                let operator = self.advance().unwrap();
                let right = self.equality()?;
                return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.term()?;

        for token in [
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.comparison()?;
                return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.factor()?;

        for token in [&TokenType::Minus, &TokenType::Plus] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.term()?;
                return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.unary()?;

        for token in [&TokenType::Star, &TokenType::Slash] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.factor()?;
                return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        for token in [&TokenType::Bang, &TokenType::Minus] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.unary()?;
                return Ok(Box::new(Expr::Unary(Unary::new(operator, right)?)));
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let next = self.advance().unwrap();

        Ok(match next.r#type {
            TokenType::False => Box::new(Expr::Value(Value::False)),
            TokenType::True => Box::new(Expr::Value(Value::True)),
            TokenType::None => Box::new(Expr::Value(Value::None)),
            TokenType::Integer(i) => Box::new(Expr::Value(Value::Integer(i))),
            TokenType::Float(f) => Box::new(Expr::Value(Value::Float(f))),
            TokenType::String(s) => Box::new(Expr::Value(Value::String(s))),
            TokenType::Identifier(s) => Box::new(Expr::Value(Value::Identifier(s))),
            TokenType::NewLine => Box::new(Expr::Value(Value::NewLine)),
            TokenType::EOF => return Err(ZeusErrorType::EOF),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                let right = self.consume(TokenType::RightParen, "expect ')' after expression")?;
                Box::new(Expr::Grouping(Grouping::new(next, expr, right)?))
            }
            e => panic!("Parsing not yet implemented: {}", e),
        })
    }

    fn consume(&mut self, expected: TokenType, msg: &str) -> Result<Token, ZeusErrorType> {
        if self.check(&expected) {
            return self.advance();
        }

        Err(ZeusErrorType::ParsingError(msg.to_owned()))
    }

    fn r#match(&mut self, token: &TokenType) -> bool {
        if self.check(token) {
            return true;
        }

        false
    }

    fn advance(&mut self) -> Result<Token, ZeusErrorType> {
        self.tokens.next().ok_or(ZeusErrorType::NoMoreTokens)
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        self.peek().is_some_and(|t| &t.r#type == token_type)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}
