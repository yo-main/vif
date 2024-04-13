use crate::error::AstError;
use vif_objects::ast;
use vif_objects::ast::Expr;
use vif_objects::ast::ExprBody;
use vif_objects::ast::Function;
use vif_objects::ast::Typing;
use vif_objects::ast::Value;
use vif_objects::ast::Variable;
use vif_scanner::Scanner;
use vif_scanner::Token;
use vif_scanner::TokenType;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    errors: Vec<AstError>,
    ast: Vec<ast::Stmt>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            ast: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn get_ast(self) -> Function {
        Function {
            name: "Main".to_owned(),
            params: Vec::new(),
            body: self.ast,
            typing: Typing::new(false),
        }
    }

    pub fn get_errors(self) -> Vec<AstError> {
        self.errors
    }

    pub fn build(&mut self) -> bool {
        loop {
            match self.declaration() {
                Ok(stmt) => self.ast.push(stmt),
                Err(AstError::EOF) => break,
                Err(err) => self.errors.push(err),
            };
        }

        self.errors.is_empty()
    }

    fn declaration(&mut self) -> Result<ast::Stmt, AstError> {
        match self.scanner.peek()? {
            t if t.r#type == TokenType::NewLine => {
                self.scanner.scan().unwrap();
                self.declaration()
            }
            t if t.r#type == TokenType::Var => self.var_declaration(),
            t if t.r#type == TokenType::Def => self.function_declaration(),
            _ => self.statement(),
        }
    }

    fn function_declaration(&mut self) -> Result<ast::Stmt, AstError> {
        self.scanner.scan()?;

        let name = match self.scanner.scan() {
            Ok(t) => match t.r#type {
                TokenType::Identifier(s) => s,
                _ => {
                    return Err(AstError::ParsingError(format!(
                        "Expected an identifier after def"
                    )))
                }
            },
            Err(e) => return Err(e.into()),
        };

        self.consume(TokenType::LeftParen, "Expect ( after function name")?;
        let mut parameters = Vec::new();

        loop {
            let mutable = match self.scanner.peek() {
                Ok(t) => match &t.r#type {
                    TokenType::Mut => {
                        self.scanner.scan()?;
                        true
                    }
                    _ => false,
                },
                _ => break,
            };

            match self.scanner.peek() {
                Ok(t) => match &t.r#type {
                    TokenType::RightParen => break,
                    TokenType::Comma => {
                        self.scanner.scan().unwrap();
                        continue;
                    }
                    TokenType::Identifier(s) => {
                        parameters.push(ast::FunctionParameter {
                            name: s.clone(),
                            typing: Typing::new(mutable),
                        });
                        self.scanner.scan().unwrap();
                    }
                    _ => return Err(AstError::ParsingError(format!("Expected a parameter name"))),
                },
                _ => break,
            };
        }

        self.consume(TokenType::RightParen, "Expect ) to close function")?;
        self.consume(TokenType::DoubleDot, "Expect : after function declaration")?;
        self.consume(
            TokenType::NewLine,
            "Expect new line after function declaration",
        )?;

        let func = Function::new(name, parameters, self.block()?);

        Ok(ast::Stmt::Function(func))
    }

    fn statement(&mut self) -> Result<ast::Stmt, AstError> {
        Ok(match self.scanner.peek() {
            Ok(t) if t.r#type == TokenType::Indent => ast::Stmt::Block(self.block()?),
            Ok(t) if t.r#type == TokenType::If => ast::Stmt::Condition(self.if_statement()?),
            Ok(t) if t.r#type == TokenType::While => ast::Stmt::While(self.while_statement()?),
            Ok(t) if t.r#type == TokenType::Return => ast::Stmt::Return(self.return_statement()?),
            Ok(t) if t.r#type == TokenType::Assert => ast::Stmt::Assert(self.assert_statement()?),
            _ => ast::Stmt::Expression(self.expression()?),
        })
    }

    fn assert_statement(&mut self) -> Result<ast::Assert, AstError> {
        //TODO: rework this to have expression value displayed when doing an assertion
        self.scanner.scan().unwrap();

        let value = match self.scanner.peek() {
            Ok(t) if t.r#type == TokenType::NewLine => {
                Box::new(Expr::new(ExprBody::Value(Value::None), Typing::new(false)))
            }
            _ => self.expression()?,
        };
        let stmt = ast::Assert { value };

        self.consume(
            TokenType::NewLine,
            "expects new line after assert statement",
        )?;

        Ok(stmt)
    }

    fn var_declaration(&mut self) -> Result<ast::Stmt, AstError> {
        self.scanner.scan()?;

        let mutable = match self.scanner.peek() {
            Ok(t) => match t.r#type {
                TokenType::Mut => {
                    self.scanner.scan()?;
                    true
                }
                _ => false,
            },
            _ => {
                return Err(AstError::ParsingError(format!(
                    "Expected an variable name, got EOF"
                )))
            }
        };

        let name = match self.scanner.scan() {
            Ok(t) => match t.r#type {
                TokenType::Identifier(s) => s,
                t => {
                    return Err(AstError::ParsingError(format!(
                        "Expected an variable name, got {}",
                        t
                    )))
                }
            },
            _ => {
                return Err(AstError::ParsingError(format!(
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

        Ok(ast::Stmt::Var(Variable::new(name, expr, mutable)))
    }

    fn unary(&mut self) -> Result<Box<Expr>, AstError> {
        for token in [&TokenType::Minus, &TokenType::Not] {
            if self.scanner.check(token) {
                self.scanner.scan().unwrap();
                let operator = if token == &TokenType::Not {
                    ast::UnaryOperator::Not
                } else {
                    ast::UnaryOperator::Minus
                };
                let right = self.unary()?;
                let typing = right.typing.clone();
                return Ok(Box::new(Expr::new(
                    ExprBody::Unary(ast::Unary { operator, right }),
                    typing,
                )));
            }
        }

        self.call()
    }

    fn return_statement(&mut self) -> Result<ast::Return, AstError> {
        self.scanner.scan().unwrap();

        let value = match self.scanner.peek() {
            Ok(t) if t.r#type == TokenType::NewLine => {
                Box::new(Expr::new(ExprBody::Value(Value::None), Typing::new(true)))
            }
            _ => self.expression()?,
        };
        let stmt = ast::Return { value };

        self.consume(
            TokenType::NewLine,
            "expects new line after return statement",
        )?;

        Ok(stmt)
    }

    fn while_statement(&mut self) -> Result<ast::While, AstError> {
        self.scanner.scan().unwrap();

        let condition = self.expression()?;
        self.consume(TokenType::DoubleDot, "Expect ':' after if condition")?;
        self.consume(TokenType::NewLine, "Expect new line after :")?;

        let stmt = self.statement()?;
        Ok(ast::While {
            condition,
            body: Box::new(stmt),
        })
    }

    fn if_statement(&mut self) -> Result<ast::Condition, AstError> {
        self.scanner.scan().unwrap();

        let expr = self.expression()?;
        self.consume(TokenType::DoubleDot, "Expect ':' after if condition")?;
        self.consume(TokenType::NewLine, "Expect new line after :")?;

        let then = Box::new(self.statement()?);

        let r#else = if self.scanner.check(&TokenType::ElIf) {
            Some(Box::new(ast::Stmt::Condition(self.if_statement()?)))
        } else if self.scanner.check(&TokenType::Else) {
            self.scanner.scan().unwrap();

            self.consume(TokenType::DoubleDot, "Expect ':' after else condition")?;
            self.consume(TokenType::NewLine, "Expect new line after :")?;

            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(ast::Condition { expr, then, r#else })
    }

    fn block(&mut self) -> Result<Vec<ast::Stmt>, AstError> {
        let mut stmts = Vec::new();
        self.scanner.scan().unwrap();

        loop {
            match self.scanner.peek() {
                Ok(t) if t.r#type == TokenType::NewLine => {
                    self.scanner.scan().unwrap();
                    continue;
                }
                Ok(t) if t.r#type == TokenType::Dedent => break,
                Ok(t) if t.r#type == TokenType::EOF => return Ok(stmts),
                _ => stmts.push(self.declaration()?),
            }
        }

        self.consume(TokenType::Dedent, "Expected end of block")?;

        Ok(stmts)
    }

    fn expression(&mut self) -> Result<Box<Expr>, AstError> {
        // if self.r#match(&TokenType::Comma) {
        //     let operator = self.advance().unwrap();
        //     let right = self.expression()?;
        //     return Ok(Box::new(Expr::Binary(Binary::new(expr, operator, right)?)));
        // }
        self.assignment()
    }
    fn assignment(&mut self) -> Result<Box<Expr>, AstError> {
        let expr = self.or()?;

        if self.scanner.check(&TokenType::Equal) {
            self.scanner.scan().unwrap();
            let value = self.assignment()?;
            let typing = expr.typing.clone();

            match expr.body {
                ExprBody::Value(Value::Variable(var)) => {
                    return Ok(Box::new(Expr::new(
                        ExprBody::Assign(ast::Assign { name: var, value }),
                        typing,
                    )))
                }
                ref e => self.errors.push(AstError::ParsingError(format!(
                    "Invalid assignement target: {}",
                    e
                ))),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Box<Expr>, AstError> {
        let left = self.and()?;

        if self.scanner.check(&TokenType::Or) {
            self.scanner.scan().unwrap();
            let right = self.or()?;
            let typing = Typing::new(left.typing.mutable & right.typing.mutable);
            return Ok(Box::new(Expr::new(
                ExprBody::Logical(ast::Logical {
                    left,
                    operator: ast::LogicalOperator::Or,
                    right,
                }),
                typing,
            )));
        };

        Ok(left)
    }

    fn and(&mut self) -> Result<Box<Expr>, AstError> {
        let left = self.equality()?;

        if self.scanner.check(&TokenType::And) {
            self.scanner.scan().unwrap();
            let right = self.and()?;
            return Ok(Box::new(Expr::new(
                ExprBody::Logical(ast::Logical {
                    left,
                    operator: ast::LogicalOperator::And,
                    right,
                }),
                Typing::new(true),
            )));
        };

        Ok(left)
    }

    fn equality(&mut self) -> Result<Box<Expr>, AstError> {
        let left = self.comparison()?;

        for token_type in [&TokenType::BangEqual, &TokenType::EqualEqual] {
            if self.scanner.check(token_type) {
                self.scanner.scan().unwrap();
                let right = self.equality()?;
                let operator = if token_type == &TokenType::BangEqual {
                    ast::Operator::BangEqual
                } else {
                    ast::Operator::Equal
                };

                return Ok(Box::new(Expr::new(
                    ExprBody::Binary(ast::Binary {
                        left,
                        operator,
                        right,
                    }),
                    Typing::new(true),
                )));
            }
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, AstError> {
        let left = self.addition()?;

        for token in [
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ] {
            if self.scanner.check(token) {
                self.scanner.scan().unwrap();
                let operator = if token == &TokenType::Greater {
                    ast::Operator::Greater
                } else if token == &TokenType::GreaterEqual {
                    ast::Operator::GreaterEqual
                } else if token == &TokenType::Less {
                    ast::Operator::Less
                } else {
                    ast::Operator::LessEqual
                };
                let right = self.comparison()?;
                return Ok(Box::new(Expr::new(
                    ExprBody::Binary(ast::Binary {
                        left,
                        operator,
                        right,
                    }),
                    Typing::new(true),
                )));
            }
        }

        Ok(left)
    }

    fn addition(&mut self) -> Result<Box<Expr>, AstError> {
        let left = self.minus()?;

        for token in [&TokenType::Plus] {
            if self.scanner.check(token) {
                self.scanner.scan().unwrap();
                let right = self.addition()?;
                return Ok(Box::new(Expr::new(
                    ExprBody::Binary(ast::Binary {
                        left,
                        operator: ast::Operator::Plus,
                        right,
                    }),
                    Typing::new(true),
                )));
            }
        }

        Ok(left)
    }

    fn minus(&mut self) -> Result<Box<Expr>, AstError> {
        let left = self.factor()?;

        for token in [&TokenType::Minus] {
            if self.scanner.check(token) {
                self.scanner.scan().unwrap();
                let right = self.minus()?;
                return Ok(Box::new(Expr::new(
                    ExprBody::Binary(ast::Binary {
                        left,
                        operator: ast::Operator::Minus,
                        right,
                    }),
                    Typing::new(true),
                )));
            }
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Box<Expr>, AstError> {
        let left = self.unary()?;

        for token in [&TokenType::Star, &TokenType::Slash, &TokenType::Modulo] {
            if self.scanner.check(token) {
                self.scanner.scan().unwrap();
                let right = self.factor()?;
                let operator = if token == &TokenType::Star {
                    ast::Operator::Multiply
                } else if token == &TokenType::Slash {
                    ast::Operator::Divide
                } else {
                    ast::Operator::Modulo
                };

                return Ok(Box::new(Expr::new(
                    ExprBody::Binary(ast::Binary {
                        left,
                        operator,
                        right,
                    }),
                    Typing::new(true),
                )));
            }
        }

        Ok(left)
    }

    fn call(&mut self) -> Result<Box<Expr>, AstError> {
        let mut expr = self.primary()?;

        loop {
            if self.scanner.check(&TokenType::LeftParen) {
                self.scanner.scan().unwrap();
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> Result<Box<Expr>, AstError> {
        let mut arguments = Vec::new();

        loop {
            match self.scanner.peek() {
                Ok(t) if t.r#type == TokenType::Comma => {
                    self.scanner.scan().unwrap();
                    arguments.push(self.expression()?);
                }
                Ok(t) if t.r#type == TokenType::RightParen => break,
                _ => arguments.push(self.expression()?),
            }
        }

        self.consume(TokenType::RightParen, "Expected ) after arguments")?;
        let typing = callee.typing.clone();

        Ok(Box::new(Expr::new(
            ExprBody::Call(ast::Call { callee, arguments }),
            typing,
        )))
    }

    fn primary(&mut self) -> Result<Box<Expr>, AstError> {
        let next = self.scanner.scan().unwrap();

        Ok(match next.r#type {
            TokenType::False => {
                Box::new(Expr::new(ExprBody::Value(Value::False), Typing::new(true)))
            }
            TokenType::True => Box::new(Expr::new(ExprBody::Value(Value::True), Typing::new(true))),
            TokenType::None => Box::new(Expr::new(ExprBody::Value(Value::None), Typing::new(true))),
            TokenType::Integer(i) => Box::new(Expr::new(
                ExprBody::Value(Value::Integer(i)),
                Typing::new(true),
            )),
            TokenType::Float(f) => Box::new(Expr::new(
                ExprBody::Value(Value::Float(f)),
                Typing::new(true),
            )),
            TokenType::String(s) => Box::new(Expr::new(
                ExprBody::Value(Value::String(s)),
                Typing::new(true),
            )),
            TokenType::Identifier(s) => Box::new(Expr::new(
                ExprBody::Value(Value::Variable(s.to_owned())),
                Typing::new(false),
            )),
            TokenType::Break => {
                self.consume(TokenType::NewLine, "Expect new line after break")?;
                Box::new(Expr::new(
                    ExprBody::LoopKeyword(ast::LoopKeyword::Break),
                    Typing::new(false),
                ))
            }
            TokenType::Continue => {
                self.consume(TokenType::NewLine, "Expect new line after continue")?;
                Box::new(Expr::new(
                    ExprBody::LoopKeyword(ast::LoopKeyword::Continue),
                    Typing::new(false),
                ))
            }
            TokenType::EOF => return Err(AstError::EOF),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                let typing = expr.typing.clone();
                self.consume(TokenType::RightParen, "expect ')' after expression")?;
                Box::new(Expr::new(
                    ExprBody::Grouping(ast::Grouping {
                        left: ast::Group::LeftParen,
                        expr,
                        right: ast::Group::RightParen,
                    }),
                    typing,
                ))
            }
            e => panic!("Parsing not yet implemented: {}", e),
        })
    }

    fn consume(&mut self, expected: TokenType, msg: &str) -> Result<Token, AstError> {
        if self.scanner.check(&expected) {
            return Ok(self.scanner.scan()?);
        }

        Err(AstError::ParsingError(msg.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use vif_objects::ast;

    use super::ast::Binary;
    use super::ast::Call;
    use super::ast::Condition;
    use super::ast::Function;
    use super::ast::FunctionParameter;
    use super::ast::Logical;
    use super::ast::LogicalOperator;
    use super::ast::Operator;
    use super::ast::Return;
    use super::ast::Stmt;
    use super::ast::Typing;
    use super::ast::Unary;
    use super::ast::UnaryOperator;
    use super::ast::Value;
    use super::ast::Variable;
    use super::Expr;
    use super::ExprBody;
    use super::Parser;
    use super::Scanner;

    #[test]
    fn simple_string() {
        let string = "\"This is a simple string\"\n";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Value(Value::String("This is a simple string".to_owned())),
                Typing::new(true)
            )))
        );
    }

    #[test]
    fn unary_expression() {
        let string = "-1";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Unary(Unary {
                    operator: UnaryOperator::Minus,
                    right: Box::new(Expr::new(
                        ExprBody::Value(Value::Integer(1)),
                        Typing::new(true)
                    ))
                }),
                Typing::new(true)
            )))
        );
    }

    #[test]
    fn var_declaration() {
        let string = "var coucou = -1\n";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Var(Variable {
                name: "coucou".to_owned(),
                typing: Typing::new(false),
                value: Box::new(Expr::new(
                    ExprBody::Unary(Unary {
                        operator: UnaryOperator::Minus,
                        right: Box::new(Expr::new(
                            ExprBody::Value(Value::Integer(1)),
                            Typing::new(true)
                        ))
                    }),
                    Typing::new(true),
                ))
            })
        );
    }

    #[test]
    fn var_mut_declaration() {
        let string = "var mut coucou = -1\n";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Var(Variable {
                name: "coucou".to_owned(),
                typing: Typing::new(true),
                value: Box::new(Expr::new(
                    ExprBody::Unary(Unary {
                        operator: UnaryOperator::Minus,
                        right: Box::new(Expr::new(
                            ExprBody::Value(Value::Integer(1)),
                            Typing::new(true)
                        ))
                    }),
                    Typing::new(true)
                ))
            })
        );
    }

    #[test]
    fn equality() {
        let string = "4 == 3+1";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Binary(Binary {
                    left: Box::new(Expr::new(
                        ExprBody::Value(Value::Integer(4)),
                        Typing::new(true)
                    )),
                    operator: Operator::Equal,
                    right: Box::new(Expr::new(
                        ExprBody::Binary(Binary {
                            left: Box::new(Expr::new(
                                ExprBody::Value(Value::Integer(3)),
                                Typing::new(true)
                            )),
                            operator: Operator::Plus,
                            right: Box::new(Expr::new(
                                ExprBody::Value(Value::Integer(1)),
                                Typing::new(true)
                            )),
                        }),
                        Typing::new(true)
                    ))
                }),
                Typing::new(true)
            )))
        );
    }

    #[test]
    fn and() {
        let string = "True and False";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Logical(Logical {
                    left: Box::new(Expr::new(ExprBody::Value(Value::True), Typing::new(true))),
                    operator: LogicalOperator::And,
                    right: Box::new(Expr::new(ExprBody::Value(Value::False), Typing::new(true))),
                }),
                Typing::new(true)
            )))
        );
    }

    #[test]
    fn or() {
        let string = "True or False";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Logical(Logical {
                    left: Box::new(Expr::new(ExprBody::Value(Value::True), Typing::new(true))),
                    operator: LogicalOperator::Or,
                    right: Box::new(Expr::new(ExprBody::Value(Value::False), Typing::new(true))),
                }),
                Typing::new(true)
            )))
        );
    }

    #[test]
    fn call() {
        let string = "my_function()";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Call(Call {
                    callee: Box::new(Expr::new(
                        ExprBody::Value(Value::Variable("my_function".to_owned())),
                        Typing::new(false)
                    )),
                    arguments: Vec::new(),
                }),
                Typing::new(false)
            )))
        );
    }

    #[test]
    fn call_with_args() {
        let string = "my_function(a, b, c)";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Call(Call {
                    callee: Box::new(Expr::new(
                        ExprBody::Value(Value::Variable("my_function".to_owned())),
                        Typing::new(false)
                    )),
                    arguments: vec![
                        Box::new(Expr::new(
                            ExprBody::Value(Value::Variable("a".to_owned())),
                            Typing::new(false)
                        )),
                        Box::new(Expr::new(
                            ExprBody::Value(Value::Variable("b".to_owned())),
                            Typing::new(false)
                        )),
                        Box::new(Expr::new(
                            ExprBody::Value(Value::Variable("c".to_owned())),
                            Typing::new(false)
                        )),
                    ]
                }),
                Typing::new(false)
            )))
        );
    }

    #[test]
    fn function_definition() {
        let string = "
            def my_function(a, b, mut c):
                return
        ";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Function(Function {
                name: "my_function".to_owned(),
                params: vec![
                    FunctionParameter {
                        name: "a".to_owned(),
                        typing: Typing::new(false)
                    },
                    FunctionParameter {
                        name: "b".to_owned(),
                        typing: Typing::new(false)
                    },
                    FunctionParameter {
                        name: "c".to_owned(),
                        typing: Typing::new(true)
                    },
                ],
                body: vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(ExprBody::Value(Value::None), Typing::new(true)))
                })],
                typing: Typing::new(false)
            })
        );
    }

    #[test]
    fn function_with_return() {
        let string = "
            def my_function(a, mut b, c):
                return 1.5
        ";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Function(Function {
                name: "my_function".to_owned(),
                params: vec![
                    FunctionParameter {
                        name: "a".to_owned(),
                        typing: Typing::new(false)
                    },
                    FunctionParameter {
                        name: "b".to_owned(),
                        typing: Typing::new(true)
                    },
                    FunctionParameter {
                        name: "c".to_owned(),
                        typing: Typing::new(false)
                    },
                ],
                body: vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::Float(1.5)),
                        Typing::new(true)
                    ))
                })],
                typing: Typing::new(false)
            })
        );
    }

    #[test]
    fn if_statement() {
        let string = "if True:\n    return \"coucou\"\n";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Typing::new(true))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Typing::new(true)
                    ))
                })])),
                r#else: None
            })
        );
    }

    #[test]
    fn if_statement_with_else() {
        let string = "if True:\n    return \"coucou\"\nelse:\n    return \"bye\"\n";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Typing::new(true))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Typing::new(true)
                    ))
                })])),
                r#else: Some(Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("bye".to_owned())),
                        Typing::new(true)
                    ))
                })])))
            })
        );
    }

    #[test]
    fn if_statement_with_elif() {
        let string = "
            if True:
                return \"coucou\"
            elif False:
                return \"bye\"
        ";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Typing::new(true))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Typing::new(true)
                    ))
                })])),
                r#else: Some(Box::new(Stmt::Condition(Condition {
                    expr: Box::new(Expr::new(ExprBody::Value(Value::False), Typing::new(true))),
                    then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                        value: Box::new(Expr::new(
                            ExprBody::Value(Value::String("bye".to_owned())),
                            Typing::new(true)
                        ))
                    })])),
                    r#else: None
                })))
            })
        );
    }

    #[test]
    fn if_statement_with_elif_else() {
        let string = "
            if True:
                return \"coucou\"
            elif False:
                return \"bye\"
            else:
                return \"hello\"
        ";
        let scanner = Scanner::new(string);
        let mut parser = Parser::new(scanner);

        let success = parser.build();

        assert!(success);
        assert_eq!(parser.ast.len(), 1);
        assert_eq!(
            parser.ast[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Typing::new(true))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Typing::new(true)
                    ))
                })])),
                r#else: Some(Box::new(Stmt::Condition(Condition {
                    expr: Box::new(Expr::new(ExprBody::Value(Value::False), Typing::new(true))),
                    then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                        value: Box::new(Expr::new(
                            ExprBody::Value(Value::String("bye".to_owned())),
                            Typing::new(true)
                        ))
                    })])),
                    r#else: Some(Box::new(Stmt::Block(vec![Stmt::Return(Return {
                        value: Box::new(Expr::new(
                            ExprBody::Value(Value::String("hello".to_owned())),
                            Typing::new(true)
                        ))
                    })])))
                })))
            })
        );
    }
}
