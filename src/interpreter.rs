use crate::ast;
use crate::tokens::{Token, TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    fn is_truthy(&self, token_type: TokenType) -> TokenType {
        match token_type {
            TokenType::None => TokenType::False,
            TokenType::Integer(0) => TokenType::False,
            TokenType::Float(0.0) => TokenType::False,
            TokenType::String(v) if v.is_empty() => TokenType::False,
            _ => TokenType::True,
        }
    }

    fn not(&self, token_type: TokenType) -> TokenType {
        match token_type {
            TokenType::False => TokenType::True,
            TokenType::True => TokenType::False,
            e => panic!("Not usage is not allowed for {}", e),
        }
    }
}

impl ast::AstVisitor for Interpreter {
    type Item = Token;

    fn visit_operator(&mut self, item: &ast::Operator) -> Self::Item {
        item.value.clone()
    }
    fn visit_literal(&mut self, item: &ast::Literal) -> Self::Item {
        item.value.clone()
    }
    fn visit_unary(&mut self, item: &ast::Unary) -> Self::Item {
        let right = item.right.accept(self);

        Token::create(match &item.operator.r#type {
            TokenType::Minus => match right.r#type {
                TokenType::Integer(v) => TokenType::Integer(v * -1),
                TokenType::Float(v) => TokenType::Float(v * -1.0),
                _ => panic!("Can't apply minus on something else than number"),
            },
            TokenType::Bang => self.not(self.is_truthy(right.r#type)),
            e => panic!("[Unary] should not be reacheable: {}", e),
        })
    }

    fn visit_binary(&mut self, item: &ast::Binary) -> Self::Item {
        let right = item.right.accept(self);
        let left = item.left.accept(self);

        Token::create(match &item.operator.r#type {
            TokenType::Minus => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(l - r),
                    TokenType::Float(r) => TokenType::Integer(l - r as i64),
                    TokenType::True => TokenType::Integer(l - 1),
                    TokenType::False => TokenType::Integer(l),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Float(l - r as f64),
                    TokenType::Float(r) => TokenType::Float(l - r),
                    TokenType::True => TokenType::Float(l - 1.0),
                    TokenType::False => TokenType::Float(l),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::True => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(1 - r),
                    TokenType::Float(r) => TokenType::Float(1.0 - r),
                    TokenType::True => TokenType::Integer(0),
                    TokenType::False => TokenType::Integer(-1),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::False => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(r),
                    TokenType::Float(r) => TokenType::Float(r),
                    TokenType::True => TokenType::Integer(-1),
                    TokenType::False => TokenType::Integer(0),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::Slash => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(l / r),
                    TokenType::Float(r) => TokenType::Integer(l / r as i64),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Float(l / r as f64),
                    TokenType::Float(r) => TokenType::Float(l / r),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::Star => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(l * r),
                    TokenType::Float(r) => TokenType::Integer(l * r as i64),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Float(l * r as f64),
                    TokenType::Float(r) => TokenType::Float(l * r),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::Plus => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(l + r),
                    TokenType::Float(r) => TokenType::Integer(l + r as i64),
                    TokenType::True => TokenType::Integer(l + 1),
                    TokenType::False => TokenType::Integer(l),
                    _ => panic!("[Binary] right expr must be a number, got {:?}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => TokenType::Float(l + r as f64),
                    TokenType::Float(r) => TokenType::Float(l + r),
                    TokenType::True => TokenType::Float(l + 1.0),
                    TokenType::False => TokenType::Float(l),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::String(l) => match right.r#type {
                    TokenType::String(r) => TokenType::String(format!("{}{}", l, r)),
                    _ => panic!("[Binary] can't add a string with {}", right),
                },
                TokenType::True => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(1 + r),
                    TokenType::Float(r) => TokenType::Float(1.0 + r),
                    TokenType::True => TokenType::Integer(2),
                    TokenType::False => TokenType::Integer(1),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::False => match right.r#type {
                    TokenType::Integer(r) => TokenType::Integer(r),
                    TokenType::Float(r) => TokenType::Float(r),
                    TokenType::True => TokenType::Integer(1),
                    TokenType::False => TokenType::Integer(0),
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] plus operator not supported on {}", left),
            },
            TokenType::Greater => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => match l > r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l > r as i64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => match l > r as f64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l > r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::GreaterEqual => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => match l >= r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l >= r as i64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => match l >= r as f64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l >= r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::Less => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => match l < r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l < r as i64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => match l < r as f64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l < r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::LessEqual => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => match l <= r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l <= r as i64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => match l <= r as f64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l <= r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::EqualEqual => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => match l == r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l == r as i64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::None => TokenType::False,
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => match l == r as f64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l == r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::None => TokenType::False,
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::String(l) => match right.r#type {
                    TokenType::String(r) => match l == r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::None => TokenType::False,
                    _ => panic!("[Binary] can't add a string with {}", right),
                },
                TokenType::None => TokenType::False,
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            TokenType::BangEqual => match left.r#type {
                TokenType::Integer(l) => match right.r#type {
                    TokenType::Integer(r) => match l != r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l != r as i64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::None => TokenType::False,
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::Float(l) => match right.r#type {
                    TokenType::Integer(r) => match l != r as f64 {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::Float(r) => match l != r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::None => TokenType::False,
                    _ => panic!("[Binary] right expr must be a number, got {}", right),
                },
                TokenType::String(l) => match right.r#type {
                    TokenType::String(r) => match l != r {
                        true => TokenType::True,
                        false => TokenType::False,
                    },
                    TokenType::None => TokenType::False,
                    _ => panic!("[Binary] can't add a string with {}", right),
                },
                TokenType::None => TokenType::None,
                _ => panic!("[Binary] minus operator not supported on {}", right),
            },
            t => panic!("[Binary] nope: {}", t),
        })
    }
    fn visit_grouping(&mut self, item: &ast::Grouping) -> Self::Item {
        item.expr.accept(self)
    }
    fn visit_expr(&mut self, item: &ast::Expr) -> Self::Item {
        match item {
            ast::Expr::Operator(v) => v.accept(self),
            ast::Expr::Binary(v) => v.accept(self),
            ast::Expr::Unary(v) => v.accept(self),
            ast::Expr::Grouping(v) => v.accept(self),
            ast::Expr::Literal(v) => v.accept(self),
        }
    }
}
