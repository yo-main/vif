use crate::ast;
use crate::errors::ZeusErrorType;
use crate::tokens::Token;
use crate::tokens::TokenType;
use std::iter::Iterator;
use std::iter::Peekable;

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn expression(&mut self) -> Result<Box<ast::Expr>, ZeusErrorType> {
        println!("-> expression");
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<ast::Expr>, ZeusErrorType> {
        println!("-> equality");
        let expr = self.comparison()?;

        for token_type in [&TokenType::BangEqual, &TokenType::EqualEqual] {
            if self.r#match(token_type) {
                let operator = self.advance().unwrap();
                let right = self.comparison()?;
                return Ok(Box::new(ast::Expr::Binary(ast::Binary::new(
                    expr, operator, right,
                ))));
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<ast::Expr>, ZeusErrorType> {
        println!("-> comparison");
        let expr = self.term()?;

        for token in [
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.term()?;
                return Ok(Box::new(ast::Expr::Binary(ast::Binary::new(
                    expr, operator, right,
                ))));
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<ast::Expr>, ZeusErrorType> {
        println!("-> term");
        let expr = self.factor()?;

        for token in [&TokenType::Minus, &TokenType::Plus] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.factor()?;
                return Ok(Box::new(ast::Expr::Binary(ast::Binary::new(
                    expr, operator, right,
                ))));
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<ast::Expr>, ZeusErrorType> {
        println!("-> factor");
        let expr = self.unary()?;

        for token in [&TokenType::Star, &TokenType::Slash] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.unary()?;
                return Ok(Box::new(ast::Expr::Binary(ast::Binary::new(
                    expr, operator, right,
                ))));
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<ast::Expr>, ZeusErrorType> {
        println!("-> unary");
        for token in [&TokenType::Bang] {
            if self.r#match(token) {
                let operator = self.advance().unwrap();
                let right = self.unary()?;
                return Ok(Box::new(ast::Expr::Unary(ast::Unary::new(operator, right))));
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<ast::Expr>, ZeusErrorType> {
        println!("-> primary");
        let next = self.advance().unwrap();

        Ok(match next.r#type {
            TokenType::False => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::True => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::None => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::Integer(_) => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::Float(_) => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::String(_) => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::Identifier(_) => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::NewLine => Box::new(ast::Expr::Literal(ast::Literal::new(next))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                let right = self.consume(TokenType::RightParen, "expect ')' after expression")?;
                Box::new(ast::Expr::Grouping(ast::Grouping::new(next, expr, right)))
            }
            e => panic!("Parsing not yet implemented: {}", e),
        })
    }

    fn consume(&mut self, expected: TokenType, msg: &str) -> Result<Token, ZeusErrorType> {
        println!("-> consume");
        if self.check(&expected) {
            return self.advance();
        }

        Err(ZeusErrorType::ParsingError(msg.to_owned()))
    }

    fn r#match(&mut self, token: &TokenType) -> bool {
        println!("-> match");
        if self.check(token) {
            return true;
        }

        false
    }

    fn advance(&mut self) -> Result<Token, ZeusErrorType> {
        println!("-> advance");
        self.tokens.next().ok_or(ZeusErrorType::NoMoreTokens)
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        println!("-> check");
        self.peek().is_some_and(|t| &t.r#type == token_type)
    }

    fn peek(&mut self) -> Option<&Token> {
        println!("-> peek");
        self.tokens.peek()
    }
}
