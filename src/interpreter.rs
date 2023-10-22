use crate::ast::{
    AstVisitor, Binary, Expr, Group, Grouping, Literal, Number, Operator, Stmt, Unary,
    UnaryOperator, Value, Variable,
};
use crate::errors::ZeusErrorType;
// use crate::tokens::{Token, TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    fn is_truthy(&self, value: Value) -> Value {
        match value {
            Value::None => Value::False,
            Value::Integer(0) => Value::False,
            Value::Float(0.0) => Value::False,
            Value::String(v) if v.is_empty() => Value::False,
            _ => Value::True,
        }
    }

    fn not(&self, value: Value) -> Value {
        match value {
            Value::False => Value::True,
            Value::True => Value::False,
            e => panic!("Not usage is not allowed for {}", e),
        }
    }

    fn print(&self, expr: Value) {
        print!("printing {}", expr);
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        statements
            .iter()
            .for_each(|e| println!("{}", e.accept(self).unwrap()));
    }
}

impl AstVisitor for Interpreter {
    type Item = Result<Value, ZeusErrorType>;

    fn visit_operator(&mut self, item: &Operator) -> Self::Item {
        Ok(Value::Operator(item.clone()))
    }

    fn visit_literal(&mut self, item: &Literal) -> Self::Item {
        Ok(match item {
            Literal::String(v) => Value::String(v.clone()),
            Literal::Indentifier(v) => Value::Variable(v.clone()),
        })
    }

    fn visit_unary(&mut self, item: &Unary) -> Self::Item {
        let right = item.right.accept(self)?;

        match &item.operator {
            UnaryOperator::Minus => match right {
                Value::Integer(i) => Ok(Value::Integer(i * -1)),
                Value::Float(f) => Ok(Value::Float(f * -1.0)),
                e => Err(ZeusErrorType::InterpreterError(format!(
                    "Can't have unary with {}",
                    e
                ))),
            },
            UnaryOperator::Bang => Ok(self.not(self.is_truthy(right))),
        }
    }

    fn visit_binary(&mut self, item: &Binary) -> Self::Item {
        let right = item.right.accept(self)?;
        let left = item.left.accept(self)?;

        match &item.operator {
            Operator::Minus => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => Ok(Value::Integer(l - r)),
                    Value::Float(r) => Ok(Value::Integer(l - r as i64)),
                    Value::True => Ok(Value::Integer(l - 1)),
                    Value::False => Ok(Value::Integer(l)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => Ok(Value::Float(l - r as f64)),
                    Value::Float(r) => Ok(Value::Float(l - r)),
                    Value::True => Ok(Value::Float(l - 1.0)),
                    Value::False => Ok(Value::Float(l)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::True => match right {
                    Value::Integer(r) => Ok(Value::Integer(1 - r)),
                    Value::Float(r) => Ok(Value::Float(1.0 - r)),
                    Value::True => Ok(Value::Integer(0)),
                    Value::False => Ok(Value::Integer(-1)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::False => match right {
                    Value::Integer(r) => Ok(Value::Integer(r)),
                    Value::Float(r) => Ok(Value::Float(r)),
                    Value::True => Ok(Value::Integer(-1)),
                    Value::False => Ok(Value::Integer(0)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] minus operator not supported on {}",
                    right
                ))),
            },
            Operator::Divide => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => Ok(Value::Integer(l / r)),
                    Value::Float(r) => Ok(Value::Integer(l / r as i64)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => Ok(Value::Float(l / r as f64)),
                    Value::Float(r) => Ok(Value::Float(l / r)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] divide operator not supported on {}",
                    right
                ))),
            },
            Operator::Multiply => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => Ok(Value::Integer(l * r)),
                    Value::Float(r) => Ok(Value::Integer(l * r as i64)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => Ok(Value::Float(l * r as f64)),
                    Value::Float(r) => Ok(Value::Float(l * r)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] multiply operator not supported on {}",
                    right
                ))),
            },
            Operator::Plus => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => Ok(Value::Integer(l + r)),
                    Value::Float(r) => Ok(Value::Integer(l + r as i64)),
                    Value::True => Ok(Value::Integer(l + 1)),
                    Value::False => Ok(Value::Integer(l)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => Ok(Value::Float(l + r as f64)),
                    Value::Float(r) => Ok(Value::Float(l + r)),
                    Value::True => Ok(Value::Float(l + 1.0)),
                    Value::False => Ok(Value::Float(l)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::String(l) => match right {
                    Value::String(r) => Ok(Value::String(format!("{}{}", l, r))),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] can't add a string with {}",
                        right
                    ))),
                },
                Value::True => match right {
                    Value::Integer(r) => Ok(Value::Integer(1 + r)),
                    Value::Float(r) => Ok(Value::Float(1.0 + r)),
                    Value::True => Ok(Value::Integer(2)),
                    Value::False => Ok(Value::Integer(1)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::False => match right {
                    Value::Integer(r) => Ok(Value::Integer(r)),
                    Value::Float(r) => Ok(Value::Float(r)),
                    Value::True => Ok(Value::Integer(1)),
                    Value::False => Ok(Value::Integer(0)),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] plus operator not supported on {}",
                    right
                ))),
            },
            Operator::Greater => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => match l > r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l > r as i64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => match l > r as f64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l > r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] greater operator not supported on {}",
                    right
                ))),
            },
            Operator::GreaterEqual => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => match l >= r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l >= r as i64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => match l >= r as f64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l >= r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] greaterEqual operator not supported on {}",
                    right
                ))),
            },
            Operator::Less => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => match l < r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l < r as i64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => match l < r as f64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l < r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] less operator not supported on {}",
                    right
                ))),
            },
            Operator::LessEqual => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => match l <= r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l <= r as i64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => match l <= r as f64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l <= r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] lessEqual operator not supported on {}",
                    right
                ))),
            },
            Operator::EqualEqual => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => match l == r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l == r as i64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    _ => Ok(Value::False),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => match l == r as f64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l == r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::None => Ok(Value::False),
                    Value::False => Ok(Value::False),
                    _ => Ok(Value::False),
                },
                Value::String(l) => match right {
                    Value::String(r) => match l == r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::None => Ok(Value::False),
                    Value::False => Ok(Value::False),
                    _ => Ok(Value::False),
                },
                Value::False => match right {
                    Value::True => Ok(Value::False),
                    Value::False => Ok(Value::True),
                    _ => Ok(Value::False),
                },
                Value::True => match right {
                    Value::True => Ok(Value::True),
                    Value::False => Ok(Value::False),
                    Value::None => Ok(Value::False),
                    _ => Ok(Value::True),
                },
                Value::None => Ok(Value::False),
                _ => Ok(Value::False),
            },
            Operator::BangEqual => match left {
                Value::Integer(l) => match right {
                    Value::Integer(r) => match l != r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l != r as i64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::None => Ok(Value::False),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::Float(l) => match right {
                    Value::Integer(r) => match l != r as f64 {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::Float(r) => match l != r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::None => Ok(Value::False),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] right expr must be a number, got {}",
                        right
                    ))),
                },
                Value::String(l) => match right {
                    Value::String(r) => match l != r {
                        true => Ok(Value::True),
                        false => Ok(Value::False),
                    },
                    Value::None => Ok(Value::False),
                    _ => Err(ZeusErrorType::InterpreterError(format!(
                        "[Binary] can't compare a string with {}",
                        right
                    ))),
                },
                Value::None => Ok(Value::None),
                _ => Err(ZeusErrorType::InterpreterError(format!(
                    "[Binary] bangEqual operator not supported on {}",
                    right
                ))),
            },
            _ => Err(ZeusErrorType::InterpreterError(format!(
                "[Binary] not implemented yet: {}",
                right
            ))),
        }
    }

    fn visit_grouping(&mut self, item: &Grouping) -> Self::Item {
        item.expr.accept(self)
    }

    fn visit_value(&mut self, item: &Value) -> Self::Item {
        Ok(item.clone())
    }

    fn visit_expr(&mut self, item: &Expr) -> Self::Item {
        match item {
            Expr::Operator(v) => v.accept(self),
            Expr::Binary(v) => v.accept(self),
            Expr::Unary(v) => v.accept(self),
            Expr::Grouping(v) => v.accept(self),
            Expr::Literal(v) => v.accept(self),
            Expr::Value(v) => v.accept(self),
        }
    }

    fn visit_variable(&mut self, item: &Variable) -> Self::Item {
        Ok(Value::Ignore)
    }

    fn visit_stmt(&mut self, item: &Stmt) -> Self::Item {
        match item {
            Stmt::Expression(e) => e.accept(self),
            Stmt::Test(e) => {
                let v = e.accept(self)?;
                self.print(v);
                Ok(Value::Ignore)
            }
            Stmt::Var(var) => var.accept(self),
        }
    }
}
