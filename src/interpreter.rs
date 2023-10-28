use crate::ast::{
    Assign, AstVisitor, Binary, BuiltIn, Call, Expr, Function, Grouping, Literal, Logical,
    LogicalOperator, Number, Operator, Stmt, Unary, UnaryOperator, UserFunction, Value, Variable,
    While,
};
use crate::builtin::{get_time, print};
use crate::environment::Environment;
use crate::errors::ZeusErrorType;
use crate::tokens::TokenType;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }

    fn is_truthy(&self, value: &Value) -> Value {
        match value {
            Value::None => Value::False,
            Value::False => Value::False,
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

    fn print(&self, values: Vec<Value>) -> Result<Value, ZeusErrorType> {
        if values.len() != 1 {
            return Err(ZeusErrorType::InterpreterError(format!(
                "print function expects only one argument"
            )));
        };
        print(format!("{}", values.first().unwrap()).as_str());
        Ok(Value::Ignore)
    }

    fn get_time(&self, values: Vec<Value>) -> Result<Value, ZeusErrorType> {
        if values.len() != 0 {
            return Err(ZeusErrorType::InterpreterError(format!(
                "get_time function does not expect any argument"
            )));
        };
        Ok(Value::Integer(get_time()))
    }

    fn execute_builtin(
        &self,
        callee: BuiltIn,
        arguments: Vec<Value>,
    ) -> Result<Value, ZeusErrorType> {
        match callee {
            BuiltIn::Print => self.print(arguments),
            BuiltIn::GetTime => self.get_time(arguments),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        statements.iter().for_each(|e| match e.accept(self) {
            Ok(_) => (),
            Err(err) => println!("error: {:?}", err),
        });
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        mut env: Option<Environment>,
    ) -> Result<Value, ZeusErrorType> {
        if let Some(mut e) = env.as_mut() {
            std::mem::swap(&mut self.env, &mut e);
        };

        self.env.start_new();

        for stmt in statements {
            stmt.accept(self)?;
        }

        self.env.close();

        if let Some(mut e) = env.as_mut() {
            std::mem::swap(&mut self.env, &mut e);
        };
        Ok(Value::Ignore)
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
            UnaryOperator::Bang => Ok(self.not(self.is_truthy(&right))),
        }
    }

    fn visit_logical(&mut self, item: &Logical) -> Self::Item {
        let left = item.left.accept(self)?;

        match item.operator {
            LogicalOperator::And => {
                match self.is_truthy(&left) {
                    Value::False => return Ok(left),
                    Value::True => (),
                    _ => (), // impossible
                };
            }
            LogicalOperator::Or => {
                match self.is_truthy(&left) {
                    Value::True => return Ok(left),
                    Value::False => (),
                    _ => (), // impossible
                };
            }
        }

        item.right.accept(self)
    }

    fn visit_call(&mut self, item: &Call) -> Self::Item {
        let mut arguments = Vec::new();
        for arg in item.arguments.iter() {
            arguments.push(arg.accept(self)?)
        }

        match item.callee.accept(self)? {
            Value::BuiltIn(v) => self.execute_builtin(v, arguments),
            Value::UserFunction(v) => {
                let mut function = match self.env.get(&v.declaration.name)? {
                    Value::UserFunction(f) => f,
                    e => {
                        return Err(ZeusErrorType::InterpreterError(format!(
                            "Not a function: {}",
                            e
                        )))
                    }
                }
                .clone();
                function.call(self, arguments)
            }
            e => {
                return Err(ZeusErrorType::InterpreterError(format!(
                    "Not a function: {}",
                    e
                )))
            }
        }
    }

    fn visit_function(&mut self, item: &Function) -> Self::Item {
        let f = UserFunction::new(item.clone());
        self.env
            .define(f.declaration.name.clone(), Value::UserFunction(f));
        Ok(Value::Ignore)
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
        match item {
            Value::Variable(name) => match self.env.get(name) {
                Ok(value) => Ok(value.clone()),
                Err(_) => Err(ZeusErrorType::InterpreterError(format!(
                    "Unknown variable {}",
                    name,
                ))),
            },
            Value::Break => Err(ZeusErrorType::Break),
            Value::Continue => Err(ZeusErrorType::Continue),
            i => Ok(i.clone()),
        }
    }

    fn visit_expr(&mut self, item: &Expr) -> Self::Item {
        match item {
            Expr::Operator(v) => v.accept(self),
            Expr::Binary(v) => v.accept(self),
            Expr::Unary(v) => v.accept(self),
            Expr::Grouping(v) => v.accept(self),
            Expr::Literal(v) => v.accept(self),
            Expr::Value(v) => v.accept(self),
            Expr::Assign(v) => v.accept(self),
            Expr::Logical(v) => v.accept(self),
            Expr::Call(v) => v.accept(self),
        }
    }

    fn visit_variable(&mut self, item: &Variable) -> Self::Item {
        let value = item.value.accept(self)?;
        self.env.define(item.name.clone(), value);
        Ok(Value::Ignore)
    }

    fn visit_assign(&mut self, item: &Assign) -> Self::Item {
        let value = item.value.accept(self)?;
        self.env.update(&item.name, value)?;
        Ok(Value::Ignore)
    }

    fn visit_condition(&mut self, item: &crate::ast::Condition) -> Self::Item {
        let condition = item.expr.accept(self)?;
        match self.is_truthy(&condition) {
            Value::True => item.then.accept(self),
            Value::False => match &item.r#else {
                Some(stmt) => stmt.accept(self),
                None => Ok(Value::Ignore),
            },
            e => Err(ZeusErrorType::InterpreterError(format!(
                "Can't evaluate if condition: {}",
                e
            ))),
        }
    }

    fn visit_while(&mut self, item: &While) -> Self::Item {
        loop {
            let cond = &mut item.condition.accept(self)?;
            match self.is_truthy(cond) {
                Value::True => match item.body.accept(self) {
                    Err(ZeusErrorType::Break) => break,
                    Err(ZeusErrorType::Continue) => continue,
                    Err(e) => return Err(e),
                    _ => (),
                },
                _ => break, // impossible
            };
        }

        Ok(Value::Ignore)
    }

    fn visit_stmt(&mut self, item: &Stmt) -> Self::Item {
        match item {
            Stmt::Expression(e) => e.accept(self),
            Stmt::Var(var) => var.accept(self),
            Stmt::Block(stmts) => self.execute_block(stmts, None),
            Stmt::Condition(c) => c.accept(self),
            Stmt::While(w) => w.accept(self),
            Stmt::Function(f) => f.accept(self),
        }
    }
}
