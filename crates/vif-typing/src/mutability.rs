use crate::error::TypingError;

use vif_objects::ast::Assert;
use vif_objects::ast::Condition;
use vif_objects::ast::Expr;
use vif_objects::ast::ExprBody;
use vif_objects::ast::Function;
use vif_objects::ast::LogicalOperator;
use vif_objects::ast::Return;
use vif_objects::ast::Stmt;
use vif_objects::ast::Value;
use vif_objects::ast::While;

pub fn check_mutability(mut function: Function) -> Result<Function, TypingError> {
    let mut mutables = Vec::new();
    check_function(&mut function, &mut mutables)?;
    Ok(function)
}

fn check_function(function: &mut Function, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    let index = mutables.len();

    for param in function.params.iter() {
        if param.mutable {
            mutables.push(param.name.clone());
        };
    }

    check_statements(&mut function.body, mutables)?;

    mutables.truncate(index);

    function.mutable = function
        .body
        .iter()
        .filter_map(|s| match s {
            Stmt::Return(r) => Some(r),
            _ => None,
        })
        .all(|r| r.value.mutable);

    if function.mutable {
        mutables.push(function.name.clone());
    }

    println!("FUNCTION {} {}", function.name, function.mutable);
    Ok(())
}

fn check_statements(stmts: &mut Vec<Stmt>, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    for stmt in stmts.iter_mut() {
        check_statement(stmt, mutables)?;
    }
    Ok(())
}

fn check_statement(stmt: &mut Stmt, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    match stmt {
        Stmt::Var(v) => {
            check_expression(&mut v.value, mutables)?;

            if v.mutable && !v.value.mutable {
                return Err(TypingError::Mutability(format!(
                    "Cannot set non mutable expression to mutable variable {}",
                    v.name
                )));
            }

            if v.mutable {
                mutables.push(v.name.clone());
            }

            Ok(())
        }
        Stmt::Function(f) => check_function(f, mutables),
        Stmt::Expression(e) => check_expression(e, mutables),
        Stmt::Block(s) => check_statements(s, mutables),
        Stmt::Condition(c) => check_condition(c, mutables),
        Stmt::While(w) => check_while(w, mutables),
        Stmt::Return(r) => check_return(r, mutables),
        Stmt::Assert(a) => check_assert(a, mutables),
    }
}

fn check_expression(expr: &mut Expr, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    match &mut expr.body {
        ExprBody::Value(Value::Variable(v)) => {
            if mutables.contains(&v) {
                expr.mutable = true;
            }
            println!("VARIABLE {} {}", v, expr.mutable);
        }
        ExprBody::Call(c) => {
            check_expression(&mut c.callee, mutables)?;
            expr.mutable = c.callee.mutable;
            for arg in c.arguments.iter_mut() {
                check_expression(arg, mutables)?;
            }
            println!("CALL {} {}", c, expr.mutable);
        }
        ExprBody::Binary(b) => {
            check_expression(&mut b.left, mutables)?;
            check_expression(&mut b.right, mutables)?;
            expr.mutable = true;
        }
        ExprBody::Unary(u) => {
            check_expression(&mut u.right, mutables)?;
            expr.mutable = u.right.mutable;
        }
        ExprBody::Grouping(g) => {
            check_expression(&mut g.expr, mutables)?;
            expr.mutable = g.expr.mutable;
        }
        ExprBody::Assign(a) => {
            check_expression(&mut a.value, mutables)?;
            expr.mutable = a.value.mutable;
        }
        ExprBody::Logical(l) => {
            check_expression(&mut l.left, mutables)?;
            check_expression(&mut l.right, mutables)?;
            match l.operator {
                LogicalOperator::And => {
                    expr.mutable = true;
                }
                LogicalOperator::Or => {
                    expr.mutable = l.left.mutable && l.right.mutable;
                }
            }
        }
        ExprBody::LoopKeyword(_) => (),
        ExprBody::Value(_) => (),
    };

    Ok(())
}

fn check_condition(cond: &mut Condition, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    check_expression(&mut cond.expr, mutables)?;
    check_statement(&mut cond.r#then, mutables)?;
    if cond.r#else.is_some() {
        check_statement(cond.r#else.as_deref_mut().unwrap(), mutables)?;
    };
    Ok(())
}

fn check_while(r#while: &mut While, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    check_expression(&mut r#while.condition, mutables)?;
    check_statement(&mut r#while.body, mutables)?;
    Ok(())
}

fn check_return(r#return: &mut Return, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    check_expression(&mut r#return.value, mutables)
}

fn check_assert(r#assert: &mut Assert, mutables: &mut Vec<String>) -> Result<(), TypingError> {
    check_expression(&mut r#assert.value, mutables)
}
