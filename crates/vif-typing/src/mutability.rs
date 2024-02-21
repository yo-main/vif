use vif_objects::ast::Assert;
use vif_objects::ast::Condition;
use vif_objects::ast::Expr;
use vif_objects::ast::ExprBody;
use vif_objects::ast::Function;
use vif_objects::ast::Return;
use vif_objects::ast::Stmt;
use vif_objects::ast::Value;
use vif_objects::ast::While;

pub fn check_function(function: &mut Function) {
    let mut mutables = Vec::new();

    check_statements(&mut function.body, &mut mutables);
}

fn check_statements<'a, 'b>(stmts: &'a mut Vec<Stmt>, mutables: &mut Vec<&'b str>)
where
    'a: 'b,
{
    stmts.iter_mut().for_each(|s| check_statement(s, mutables));
}

fn check_statement<'a, 'b>(stmt: &'a mut Stmt, mutables: &mut Vec<&'b str>)
where
    'a: 'b,
{
    match stmt {
        Stmt::Var(v) => {
            if v.mutable {
                mutables.push(v.name.as_str());
            }
        }
        Stmt::Function(f) => check_function(f),
        Stmt::Expression(e) => check_expression(e, mutables),
        Stmt::Block(s) => check_statements(s, mutables),
        Stmt::Condition(c) => check_condition(c, mutables),
        Stmt::While(w) => check_while(w, mutables),
        Stmt::Return(r) => check_return(r, mutables),
        Stmt::Assert(a) => check_assert(a, mutables),
    }
}

fn check_expression<'a, 'b>(expr: &'a mut Expr, mutables: &'b mut Vec<&str>)
where
    'a: 'b,
{
    match &mut expr.body {
        ExprBody::Value(Value::Variable(v)) => {
            if mutables.contains(&v.name.as_str()) {
                v.mutable = true;
            }
        }
        _ => (),
    }
}

fn check_condition<'a, 'b>(cond: &'a mut Condition, mutables: &'b mut Vec<&'a str>)
where
    'a: 'b,
{
    check_expression(&mut cond.expr, mutables);
    check_statement(&mut cond.r#then, mutables);
    if cond.r#else.is_some() {
        check_statement(cond.r#else.as_deref_mut().unwrap(), mutables);
    }
}

fn check_while<'a, 'b>(r#while: &'a mut While, mutables: &mut Vec<&'b str>)
where
    'a: 'b,
{
    check_expression(&mut r#while.condition, mutables);
    check_statement(&mut r#while.body, mutables);
}

fn check_return<'a, 'b>(r#return: &'a mut Return, mutables: &'b mut Vec<&str>)
where
    'a: 'b,
{
    check_expression(&mut r#return.value, mutables);
}

fn check_assert<'a, 'b>(r#assert: &'a mut Assert, mutables: &'b mut Vec<&str>)
where
    'a: 'b,
{
    check_expression(&mut r#assert.value, mutables);
}
