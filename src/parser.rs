use clap::error::Result;

use crate::ast::{
    Assign, Binary, Condition, Expr, Grouping, Literal, Logical, Stmt, Unary, Value, Variable,
    While,
};
use crate::errors::ZeusErrorType;
use crate::tokenizer::Tokenizer;
use crate::tokens::Token;
use crate::tokens::TokenType;
use std::iter::Iterator;
use std::iter::Peekable;

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    pub statements: Vec<Stmt>,
    pub errors: Vec<ZeusErrorType>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            statements: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> bool {
        loop {
            match self.declaration() {
                Ok(stmt) => self.statements.push(stmt),
                Err(ZeusErrorType::EOF) => break,
                Err(err) => {
                    self.parse_error()
                        .expect(&format!("Could not recover from error: {:?}", err));
                    self.errors.push(err);
                }
            };
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
                Some(t) if t.r#type == TokenType::Var => return Ok(()),
                Some(t) if t.r#type == TokenType::Const => return Ok(()),
                _ => self.advance()?,
            };
        }
    }

    fn declaration(&mut self) -> Result<Stmt, ZeusErrorType> {
        match self.peek() {
            Some(t) if t.r#type == TokenType::Var => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ZeusErrorType> {
        self.advance().unwrap();

        let name = match self.peek() {
            Some(t) => match &t.r#type {
                TokenType::Identifier(_) => self.advance().unwrap(),
                t => {
                    return Err(ZeusErrorType::ParsingError(format!(
                        "Expected an variable name, got {}",
                        t
                    )))
                }
            },
            _ => {
                return Err(ZeusErrorType::ParsingError(format!(
                    "Expected an variable name, got EOF"
                )))
            }
        };

        self.consume(TokenType::Equal, "Expected an =")?;
        let expr = self.expression()?;
        self.consume(
            TokenType::NewLine,
            "Expected new line after variable declaration",
        )?;

        Ok(Stmt::Var(Variable::new(name, expr)?))
    }

    fn statement(&mut self) -> Result<Stmt, ZeusErrorType> {
        Ok(match self.peek() {
            Some(t) if t.r#type == TokenType::At => Stmt::Test(self.test_statement()?),
            Some(t) if t.r#type == TokenType::Indent => Stmt::Block(self.block()?),
            Some(t) if t.r#type == TokenType::If => Stmt::Condition(self.if_statement()?),
            Some(t) if t.r#type == TokenType::While => Stmt::While(self.while_statement()?),
            _ => Stmt::Expression(self.expression()?),
        })
    }

    fn while_statement(&mut self) -> Result<While, ZeusErrorType> {
        self.advance().unwrap();

        let condition = self.expression()?;
        self.consume(TokenType::DoubleDot, "Expect ':' after if condition")?;
        self.consume(TokenType::NewLine, "Expect new line after :")?;

        let stmt = self.statement()?;
        Ok(While::new(condition, Box::new(stmt)))
    }

    fn if_statement(&mut self) -> Result<Condition, ZeusErrorType> {
        self.advance().unwrap();

        let expr = self.expression()?;
        self.consume(TokenType::DoubleDot, "Expect ':' after if condition")?;
        self.consume(TokenType::NewLine, "Expect new line after :")?;

        let then = Box::new(self.statement()?);
        let r#else = match self.check(&TokenType::Else) {
            true => {
                self.advance().unwrap();

                self.consume(TokenType::DoubleDot, "Expect ':' after else condition")?;
                self.consume(TokenType::NewLine, "Expect new line after :")?;

                Some(Box::new(self.statement()?))
            }
            false => None,
        };

        Ok(Condition::new(expr, then, r#else))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ZeusErrorType> {
        let mut stmts = Vec::new();
        self.advance().unwrap();

        while !self.check(&TokenType::Dedent) {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenType::Dedent, "Expected end of block")?;

        Ok(stmts)
    }

    fn test_statement(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        self.advance().unwrap();
        let expr = self.expression();
        self.consume(TokenType::NewLine, "Expect new line after expression")?;
        return expr;
    }

    fn expression(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        // if self.r#match(&TokenType::Comma) {
        //     let operator = self.advance().unwrap();
        //     let right = self.expression()?;
        //     return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
        // }
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.or()?;

        if self.r#match(&TokenType::Equal) {
            self.advance().unwrap();
            let value = self.assignment()?;

            match *expr {
                Expr::Value(Value::Variable(var)) => {
                    return Ok(Box::new(Expr::Assign(Assign::new(var, value)?)))
                }
                ref e => self.errors.push(ZeusErrorType::ParsingError(format!(
                    "Invalid assignement target: {}",
                    e
                ))),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.and()?;

        if self.check(&TokenType::Or) {
            let operator = self.advance().unwrap();
            let right = self.or()?;
            return Ok(Box::new(Expr::Logical(Logical::new(
                expr, operator, right,
            )?)));
        };

        Ok(expr)
    }

    fn and(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.equality()?;

        if self.check(&TokenType::And) {
            let operator = self.advance().unwrap();
            let right = self.and()?;
            return Ok(Box::new(Expr::Logical(Logical::new(
                expr, operator, right,
            )?)));
        };

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
        let expr = self.addition()?;

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

    fn addition(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.minus()?;

        for token in [&TokenType::Plus] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.addition()?;
                return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
            }
        }

        Ok(expr)
    }

    fn minus(&mut self) -> Result<Box<Expr>, ZeusErrorType> {
        let expr = self.factor()?;

        for token in [&TokenType::Minus] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.minus()?;
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
            TokenType::Identifier(s) => Box::new(Expr::Value(Value::Variable(s))),
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
