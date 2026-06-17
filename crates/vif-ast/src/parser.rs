use crate::error::{AstError, SyntaxError};
use crate::objects::{
    Assert, Assign, Binary, Call, Condition, Entrypoint, Expr, ExprBody, Function,
    FunctionParameter, Logical, LogicalOperator, LoopKeyword, Operator, Return, Stmt,
    TypeAnnotation, Unary, UnaryOperator, Value, Variable, While,
};
use vif_loader::log;
use vif_scanner::Scanner;
use vif_scanner::TokenType;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(scanner: &mut Scanner) -> Result<Entrypoint, Vec<AstError>> {
        let parser = Self {};

        let mut ast = vec![];
        let mut errors = vec![];

        loop {
            match parser.declaration(scanner) {
                Ok(stmt) => ast.push(stmt),
                Err(AstError::EOF) => break,
                Err(err) => {
                    log::error!("Parsing error: {:?}", err);
                    errors.push(err)
                }
            };
        }

        match errors.is_empty() {
            true => Ok(Entrypoint { body: ast }),
            false => Err(errors),
        }
    }

    fn consume(
        &self,
        scanner: &mut Scanner,
        expected: TokenType,
        error_msg: &str,
    ) -> Result<TokenType, AstError> {
        if scanner.check(&expected) {
            return Ok(scanner.scan()?);
        }

        Err(SyntaxError::new(
            error_msg.to_owned(),
            scanner.get_span().clone(),
        ))
    }

    fn declaration(&self, scanner: &mut Scanner) -> Result<Stmt, AstError> {
        match scanner.peek()? {
            TokenType::NewLine => {
                scanner.consume()?;
                self.declaration(scanner)
            }
            TokenType::Var => self.var_declaration(scanner),
            TokenType::Def => self.function_declaration(scanner),
            _ => self.statement(scanner),
        }
    }

    fn statement(&self, scanner: &mut Scanner) -> Result<Stmt, AstError> {
        Ok(match scanner.peek()? {
            TokenType::Indent => Stmt::Block(self.block(scanner)?),
            TokenType::If => Stmt::Condition(self.if_statement(scanner)?),
            TokenType::While => Stmt::While(self.while_statement(scanner)?),
            TokenType::Return => Stmt::Return(self.return_statement(scanner)?),
            TokenType::Assert => Stmt::Assert(self.assert_statement(scanner)?),
            _ => Stmt::Expression(self.expression(scanner)?),
        })
    }

    fn assert_statement(&self, scanner: &mut Scanner) -> Result<Assert, AstError> {
        //TODO: rework this to have expression value displayed when doing an assertion
        scanner.consume()?;

        let value = match scanner.peek()? {
            TokenType::NewLine => Box::new(Expr::new(
                ExprBody::Value(Value::None),
                scanner.get_span().clone(),
            )),
            _ => self.expression(scanner)?,
        };
        let stmt = Assert { value };

        self.consume(
            scanner,
            TokenType::NewLine,
            "expects new line after assert statement",
        )?;

        Ok(stmt)
    }

    fn if_statement(&self, scanner: &mut Scanner) -> Result<Condition, AstError> {
        scanner.consume()?;

        let expr = self.expression(scanner)?;
        self.consume(
            scanner,
            TokenType::DoubleDot,
            "Expect ':' after if condition",
        )?;
        self.consume(scanner, TokenType::NewLine, "Expect new line after :")?;

        let then = Box::new(self.statement(scanner)?);

        let r#else = match scanner.peek()? {
            TokenType::ElIf => Some(Box::new(Stmt::Condition(self.if_statement(scanner)?))),
            TokenType::Else => {
                scanner.consume()?;

                self.consume(
                    scanner,
                    TokenType::DoubleDot,
                    "Expect ':' after else condition",
                )?;
                self.consume(scanner, TokenType::NewLine, "Expect new line after :")?;

                Some(Box::new(self.statement(scanner)?))
            }
            _ => None,
        };

        Ok(Condition { expr, then, r#else })
    }

    fn while_statement(&self, scanner: &mut Scanner) -> Result<While, AstError> {
        scanner.consume()?;

        let condition = self.expression(scanner)?;
        self.consume(
            scanner,
            TokenType::DoubleDot,
            "Expect ':' after if condition",
        )?;
        self.consume(scanner, TokenType::NewLine, "Expect new line after :")?;

        let stmt = self.statement(scanner)?;

        Ok(While {
            condition,
            body: Box::new(stmt),
        })
    }

    fn return_statement(&self, scanner: &mut Scanner) -> Result<Return, AstError> {
        scanner.consume()?;

        let value = match scanner.peek()? {
            TokenType::NewLine => Box::new(Expr::new(
                ExprBody::Value(Value::None),
                scanner.get_span().clone(),
            )),
            _ => self.expression(scanner)?,
        };

        let stmt = Return::new(value);

        self.consume(
            scanner,
            TokenType::NewLine,
            "expects new line after return statement",
        )?;

        Ok(stmt)
    }

    fn function_declaration(&self, scanner: &mut Scanner) -> Result<Stmt, AstError> {
        scanner.scan()?;

        let name = match scanner.scan()? {
            TokenType::ValueIdentifier(s) => s,
            _ => {
                return Err(SyntaxError::new(
                    format!("Expected an identifier after def"),
                    scanner.get_span().clone(),
                ))
            }
        };

        self.consume(
            scanner,
            TokenType::LeftParen,
            "Expect ( after function name",
        )?;

        let mut parameters = Vec::new();

        loop {
            let mutable = scanner.check_and_consume(&TokenType::Mut)?;

            match scanner.peek()? {
                TokenType::RightParen => break,
                TokenType::Comma => {
                    scanner.consume()?;
                    continue;
                }
                TokenType::ValueIdentifier(s) => {
                    let func_name = s.clone();

                    scanner.consume()?;

                    let annotation = self.get_annotation(scanner)?;

                    parameters.push(FunctionParameter::new(func_name, mutable, annotation));
                }
                _ => {
                    return Err(SyntaxError::new(
                        format!("Expected a parameter name"),
                        scanner.get_span().clone(),
                    ))
                }
            };
        }

        self.consume(scanner, TokenType::RightParen, "Expect ) to close function")?;
        self.consume(
            scanner,
            TokenType::DoubleDot,
            "Expect : after function declaration",
        )?;
        self.consume(
            scanner,
            TokenType::NewLine,
            "Expect new line after function declaration",
        )?;

        let func = Function::new(name, parameters, self.block(scanner)?);

        Ok(Stmt::Function(func))
    }

    fn get_annotation(&self, scanner: &mut Scanner) -> Result<Option<TypeAnnotation>, AstError> {
        if scanner.check(&TokenType::DoubleDot) {
            scanner.consume()?;

            let annotation = match scanner.peek()? {
                TokenType::Int => TypeAnnotation::Int,
                TokenType::Bool => TypeAnnotation::Bool,
                TokenType::Str => TypeAnnotation::String,
                TokenType::Float => TypeAnnotation::Float,
                t => {
                    return Err(SyntaxError::new(
                        format!("Not a type: {t}"),
                        scanner.get_span().clone(),
                    ))
                }
            };

            scanner.scan()?;
            return Ok(Some(annotation));
        }

        return Ok(None);
    }

    fn var_declaration(&self, scanner: &mut Scanner) -> Result<Stmt, AstError> {
        scanner.scan()?;

        let mutable = scanner.check_and_consume(&TokenType::Mut)?;

        let name = match scanner.scan()? {
            TokenType::ValueIdentifier(s) => s,
            t => {
                return Err(SyntaxError::new(
                    format!("Expected an variable name, got {}", t),
                    scanner.get_span().clone(),
                ))
            }
        };

        let annotation = self.get_annotation(scanner)?;

        self.consume(scanner, TokenType::Equal, "Expected an =")?;
        let expr = self.expression(scanner)?;

        self.consume(
            scanner,
            TokenType::NewLine,
            "Expected new line after variable declaration",
        )?;

        Ok(Stmt::Var(Variable::new(name, expr, mutable, annotation)))
    }

    fn block(&self, scanner: &mut Scanner<'_>) -> Result<Vec<Stmt>, AstError> {
        let mut stmts = Vec::new();
        scanner.check_and_consume(&TokenType::Indent)?;

        loop {
            println!("PEEK {:?}", scanner.peek());
            match scanner.peek()? {
                TokenType::NewLine => {
                    scanner.consume()?;
                    continue;
                }
                TokenType::Dedent => {
                    scanner.consume()?;
                    return Ok(stmts);
                }
                TokenType::EOF => return Ok(stmts),
                _ => stmts.push(self.declaration(scanner)?),
            }
        }
    }

    fn expression(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        self.assignment(scanner)
    }

    fn assignment(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let expr = self.or(scanner)?;

        if scanner.check_and_consume(&TokenType::Equal)? {
            let value = self.assignment(scanner)?;

            match expr.body {
                ExprBody::Value(Value::Variable(var)) => {
                    return Ok(Box::new(Expr::new(
                        ExprBody::Assign(Assign { name: var, value }),
                        scanner.get_span().clone(),
                    )))
                }
                ref e => {
                    return Err(SyntaxError::new(
                        format!("Invalid assignement target: {}", e),
                        scanner.get_span().clone(),
                    ))
                }
            };
        }

        Ok(expr)
    }

    fn or(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let left = self.and(scanner)?;

        if scanner.check_and_consume(&TokenType::Or)? {
            let right = self.or(scanner)?;
            return Ok(Box::new(Expr::new(
                ExprBody::Logical(Logical {
                    left,
                    operator: LogicalOperator::Or,
                    right,
                }),
                scanner.get_span().clone(),
            )));
        };

        Ok(left)
    }

    fn and(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let left = self.equality(scanner)?;

        if scanner.check_and_consume(&TokenType::And)? {
            let right = self.and(scanner)?;
            return Ok(Box::new(Expr::new(
                ExprBody::Logical(Logical {
                    left,
                    operator: LogicalOperator::And,
                    right,
                }),
                scanner.get_span().clone(),
            )));
        };

        Ok(left)
    }

    fn equality(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let left = self.comparison(scanner)?;

        let operator = match scanner.peek()? {
            TokenType::BangEqual => Operator::BangEqual,
            TokenType::EqualEqual => Operator::EqualEqual,
            _ => return Ok(left),
        };

        scanner.consume()?;

        let right = self.equality(scanner)?;

        return Ok(Box::new(Expr::new(
            ExprBody::Binary(Binary {
                left,
                operator,
                right,
            }),
            scanner.get_span().clone(),
        )));
    }

    fn comparison(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let left = self.addition(scanner)?;

        let operator = match scanner.peek()? {
            TokenType::Greater => Operator::Greater,
            TokenType::GreaterEqual => Operator::GreaterEqual,
            TokenType::Less => Operator::Less,
            TokenType::LessEqual => Operator::LessEqual,
            _ => return Ok(left),
        };

        scanner.consume()?;

        let right = self.comparison(scanner)?;
        return Ok(Box::new(Expr::new(
            ExprBody::Binary(Binary {
                left,
                operator,
                right,
            }),
            scanner.get_span().clone(),
        )));
    }

    fn addition(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let left = self.minus(scanner)?;

        if scanner.check_and_consume(&TokenType::Plus)? {
            let right = self.addition(scanner)?;
            return Ok(Box::new(Expr::new(
                ExprBody::Binary(Binary {
                    left,
                    operator: Operator::Plus,
                    right,
                }),
                scanner.get_span().clone(),
            )));
        }

        Ok(left)
    }

    fn minus(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let left = self.factor(scanner)?;

        if scanner.check(&TokenType::Minus) {
            scanner.scan().unwrap();
            let right = self.minus(scanner)?;
            return Ok(Box::new(Expr::new(
                ExprBody::Binary(Binary {
                    left,
                    operator: Operator::Minus,
                    right,
                }),
                scanner.get_span().clone(),
            )));
        }

        Ok(left)
    }

    fn factor(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let left = self.unary(scanner)?;

        let operator = match scanner.peek()? {
            TokenType::Star => Operator::Multiply,
            TokenType::Slash => Operator::Divide,
            TokenType::Modulo => Operator::Modulo,
            _ => return Ok(left),
        };

        scanner.consume()?;

        let right = self.factor(scanner)?;

        return Ok(Box::new(Expr::new(
            ExprBody::Binary(Binary {
                left,
                operator,
                right,
            }),
            scanner.get_span().clone(),
        )));
    }

    fn unary(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let operator = match scanner.peek()? {
            TokenType::Minus => UnaryOperator::Minus,
            TokenType::Not => UnaryOperator::Not,
            _ => return self.call(scanner),
        };

        scanner.consume()?;

        let right = self.unary(scanner)?;

        return Ok(Box::new(Expr::new(
            ExprBody::Unary(Unary { operator, right }),
            scanner.get_span().clone(),
        )));
    }

    fn call(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let mut expr = self.primary(scanner)?;

        loop {
            if scanner.check_and_consume(&TokenType::LeftParen)? {
                expr = self.finish_call(scanner, expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&self, scanner: &mut Scanner, callee: Box<Expr>) -> Result<Box<Expr>, AstError> {
        let mut arguments = Vec::new();

        loop {
            match scanner.peek()? {
                TokenType::Comma => {
                    scanner.consume()?;
                    arguments.push(self.expression(scanner)?);
                }
                TokenType::RightParen => break,
                _ => arguments.push(self.expression(scanner)?),
            }
        }

        self.consume(scanner, TokenType::RightParen, "Expected ) after arguments")?;

        Ok(Box::new(Expr::new(
            ExprBody::Call(Call { callee, arguments }),
            scanner.get_span().clone(),
        )))
    }

    fn primary(&self, scanner: &mut Scanner) -> Result<Box<Expr>, AstError> {
        let next = scanner.scan()?;

        Ok(match next {
            TokenType::False => Box::new(Expr::new(
                ExprBody::Value(Value::False),
                scanner.get_span().clone(),
            )),
            TokenType::True => Box::new(Expr::new(
                ExprBody::Value(Value::True),
                scanner.get_span().clone(),
            )),
            TokenType::None => Box::new(Expr::new(
                ExprBody::Value(Value::None),
                scanner.get_span().clone(),
            )),
            TokenType::ValueInteger(i) => Box::new(Expr::new(
                ExprBody::Value(Value::Integer(i)),
                scanner.get_span().clone(),
            )),
            TokenType::ValueFloat(f) => Box::new(Expr::new(
                ExprBody::Value(Value::Float(f)),
                scanner.get_span().clone(),
            )),
            TokenType::ValueString(s) => Box::new(Expr::new(
                ExprBody::Value(Value::String(s)),
                scanner.get_span().clone(),
            )),
            TokenType::ValueIdentifier(s) => Box::new(Expr::new(
                ExprBody::Value(Value::Variable(s.to_owned())),
                scanner.get_span().clone(),
            )),
            TokenType::Break => {
                self.consume(scanner, TokenType::NewLine, "Expect new line after break")?;
                Box::new(Expr::new(
                    ExprBody::LoopKeyword(LoopKeyword::Break),
                    scanner.get_span().clone(),
                ))
            }
            TokenType::Continue => {
                self.consume(
                    scanner,
                    TokenType::NewLine,
                    "Expect new line after continue",
                )?;
                Box::new(Expr::new(
                    ExprBody::LoopKeyword(LoopKeyword::Continue),
                    scanner.get_span().clone(),
                ))
            }
            TokenType::EOF => return Err(AstError::EOF),
            TokenType::LeftParen => {
                let expr = self.expression(scanner)?;
                self.consume(
                    scanner,
                    TokenType::RightParen,
                    "expect ')' after expression",
                )?;

                expr
            }
            e => panic!("Parsing not yet implemented: {}", e),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::TypeAnnotation;

    use super::Binary;
    use super::Call;
    use super::Condition;
    use super::Expr;
    use super::ExprBody;
    use super::Function;
    use super::FunctionParameter;
    use super::Logical;
    use super::LogicalOperator;
    use super::Operator;
    use super::Parser;
    use super::Return;
    use super::Scanner;
    use super::Stmt;
    use super::Unary;
    use super::UnaryOperator;
    use super::Value;
    use super::Variable;
    use vif_loader::setup_logging;
    use vif_objects::span::Span;

    #[test]
    fn simple_string() {
        let string = "\"This is a simple string\"\n";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Value(Value::String("This is a simple string".to_owned())),
                Span::new(1, 25)
            )))
        );
    }

    #[test]
    fn unary_expression() {
        let string = "-1";

        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();

        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Unary(Unary {
                    operator: UnaryOperator::Minus,
                    right: Box::new(Expr::new(
                        ExprBody::Value(Value::Integer(1)),
                        Span::new(1, 2)
                    ))
                }),
                Span::new(1, 2)
            )))
        );
    }

    #[test]
    fn var_declaration() {
        let string = "var coucou = -1\n";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Var(Variable {
                name: "coucou".to_owned(),
                value: Box::new(Expr::new(
                    ExprBody::Unary(Unary {
                        operator: UnaryOperator::Minus,
                        right: Box::new(Expr::new(
                            ExprBody::Value(Value::Integer(1)),
                            Span::new(1, 15)
                        ))
                    }),
                    Span::new(1, 16)
                )),
                mutable: false,
                annotation: None
            })
        );
    }

    #[test]
    fn var_mut_declaration() {
        let string = "var mut coucou = -1\n";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Var(Variable {
                name: "coucou".to_owned(),
                value: Box::new(Expr::new(
                    ExprBody::Unary(Unary {
                        operator: UnaryOperator::Minus,
                        right: Box::new(Expr::new(
                            ExprBody::Value(Value::Integer(1)),
                            Span::new(1, 19)
                        ))
                    }),
                    Span::new(1, 20)
                )),
                mutable: true,
                annotation: None
            })
        );
    }

    #[test]
    fn equality() {
        let string = "4 == 3+1";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Binary(Binary {
                    left: Box::new(Expr::new(
                        ExprBody::Value(Value::Integer(4)),
                        Span::new(1, 1)
                    )),
                    operator: Operator::EqualEqual,
                    right: Box::new(Expr::new(
                        ExprBody::Binary(Binary {
                            left: Box::new(Expr::new(
                                ExprBody::Value(Value::Integer(3)),
                                Span::new(1, 6)
                            )),
                            operator: Operator::Plus,
                            right: Box::new(Expr::new(
                                ExprBody::Value(Value::Integer(1)),
                                Span::new(1, 8)
                            )),
                        }),
                        Span::new(1, 8)
                    ))
                }),
                Span::new(1, 8)
            )))
        );
    }

    #[test]
    fn and() {
        let string = "True and False";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Logical(Logical {
                    left: Box::new(Expr::new(ExprBody::Value(Value::True), Span::new(1, 4))),
                    operator: LogicalOperator::And,
                    right: Box::new(Expr::new(ExprBody::Value(Value::False), Span::new(1, 14))),
                }),
                Span::new(1, 14)
            )))
        );
    }

    #[test]
    fn or() {
        let string = "True or False";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Logical(Logical {
                    left: Box::new(Expr::new(ExprBody::Value(Value::True), Span::new(1, 4))),
                    operator: LogicalOperator::Or,
                    right: Box::new(Expr::new(ExprBody::Value(Value::False), Span::new(1, 13))),
                }),
                Span::new(1, 13)
            )))
        );
    }

    #[test]
    fn call() {
        let string = "my_function()";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Call(Call {
                    callee: Box::new(Expr::new(
                        ExprBody::Value(Value::Variable("my_function".to_owned())),
                        Span::new(1, 11)
                    )),
                    arguments: Vec::new(),
                }),
                Span::new(1, 13)
            )))
        );
    }

    #[test]
    fn call_with_args() {
        let string = "my_function(a, b, c)";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Expression(Box::new(Expr::new(
                ExprBody::Call(Call {
                    callee: Box::new(Expr::new(
                        ExprBody::Value(Value::Variable("my_function".to_owned())),
                        Span::new(1, 11)
                    )),
                    arguments: vec![
                        Box::new(Expr::new(
                            ExprBody::Value(Value::Variable("a".to_owned())),
                            Span::new(1, 13)
                        )),
                        Box::new(Expr::new(
                            ExprBody::Value(Value::Variable("b".to_owned())),
                            Span::new(1, 16)
                        )),
                        Box::new(Expr::new(
                            ExprBody::Value(Value::Variable("c".to_owned())),
                            Span::new(1, 19)
                        )),
                    ]
                }),
                Span::new(1, 20)
            )))
        );
    }

    #[test]
    fn function_definition() {
        let string = "
            def my_function(a, b, mut c):
                return
        ";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Function(Function {
                name: "my_function".to_owned(),
                params: vec![
                    FunctionParameter {
                        name: "a".to_owned(),
                        mutable: false,
                        annotation: None
                    },
                    FunctionParameter {
                        name: "b".to_owned(),
                        mutable: false,
                        annotation: None
                    },
                    FunctionParameter {
                        name: "c".to_owned(),
                        mutable: true,
                        annotation: None
                    },
                ],
                body: vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(ExprBody::Value(Value::None), Span::new(3, 23)))
                })],
            })
        );
    }

    #[test]
    fn function_with_return() {
        let string = "
            def my_function(a, mut b, c: int):
                return 1.5
        ";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Function(Function {
                name: "my_function".to_owned(),
                params: vec![
                    FunctionParameter {
                        name: "a".to_owned(),
                        mutable: false,
                        annotation: None
                    },
                    FunctionParameter {
                        name: "b".to_owned(),
                        mutable: true,
                        annotation: None
                    },
                    FunctionParameter {
                        name: "c".to_owned(),
                        mutable: false,
                        annotation: Some(TypeAnnotation::Int)
                    },
                ],
                body: vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::Float(1.5)),
                        Span::new(3, 26)
                    ))
                })],
            })
        );
        // assert_eq!(1, 2);
    }

    #[test]
    fn if_statement() {
        let string = "if True:\n    return \"coucou\"\n";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Span::new(1, 7))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Span::new(2, 19)
                    ))
                })])),
                r#else: None
            })
        );
    }

    #[test]
    fn if_statement_with_else() {
        let string = "if True:\n    return \"coucou\"\nelse:\n    return \"bye\"\n";
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Span::new(1, 7))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Span::new(2, 19)
                    ))
                })])),
                r#else: Some(Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("bye".to_owned())),
                        Span::new(4, 16)
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
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Span::new(2, 19))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Span::new(3, 31)
                    ))
                })])),
                r#else: Some(Box::new(Stmt::Condition(Condition {
                    expr: Box::new(Expr::new(ExprBody::Value(Value::False), Span::new(4, 22))),
                    then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                        value: Box::new(Expr::new(
                            ExprBody::Value(Value::String("bye".to_owned())),
                            Span::new(5, 28)
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
        let mut scanner = Scanner::new(string);

        let result = Parser::build(&mut scanner);

        assert!(result.is_ok());

        let entrypoint = result.unwrap();
        assert_eq!(entrypoint.body.len(), 1);

        assert_eq!(
            entrypoint.body[0],
            Stmt::Condition(Condition {
                expr: Box::new(Expr::new(ExprBody::Value(Value::True), Span::new(2, 19))),
                then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                    value: Box::new(Expr::new(
                        ExprBody::Value(Value::String("coucou".to_owned())),
                        Span::new(3, 31)
                    ))
                })])),
                r#else: Some(Box::new(Stmt::Condition(Condition {
                    expr: Box::new(Expr::new(ExprBody::Value(Value::False), Span::new(4, 22))),
                    then: Box::new(Stmt::Block(vec![Stmt::Return(Return {
                        value: Box::new(Expr::new(
                            ExprBody::Value(Value::String("bye".to_owned())),
                            Span::new(5, 28)
                        ))
                    })])),
                    r#else: Some(Box::new(Stmt::Block(vec![Stmt::Return(Return {
                        value: Box::new(Expr::new(
                            ExprBody::Value(Value::String("hello".to_owned())),
                            Span::new(7, 30)
                        ))
                    })])))
                })))
            })
        );
    }
}
