/*
This module checks that mutability is coherent through the whole program.
If a variable is declared as mutable, we need to ensure that only mutable values can be assigned it.
It will also ensure that a variable not declared a mutable cannot be mutated.

It's okay to pass a mutable value to a const variable. The opposite is not.

First, what is a mutable value ?
It's a core type (int, string, bool...) or a variable declared as mutable.
Basically everything is mutable until it lands into a variable that is not mutable.
From that moment, the value hold by the variable cannot be mutated.

The goal of that module is to ensure this statement is true at compile time.

And the complexity hides into functions, their parameters and their return value.

A function's parameter can be mutable or not. We cannot pass a const variable to a mutable parameter,
otherwise the value could be modifed from inside the function and we don't want that.

A function's return value must also be defined as mutable or not. If not, how do we know if we can
store in a mutable variable the result from a function. Say that the function the value from a const parameter,
if we allow storing that return in a mutable variable, it'll be modified. And the variable that hold the const
value won't expect that.

Example
```vif
def hello(s):
    return s

var const_var = "world"
var mut mut_var = hello(const_var)

mut_var = "hahaha" ## should not be allowed
```

The module parses the AST and keeps track of all variables and functions in a `References` object.
Every time a variable is assigned a new value, we check that the value being assigned is in
that reference object and is mutable.

For a function to be "considered" as mutable, all of the possible return values from that function
must be mutable. If not, the function return value is not considered as mutable.

Initially we don't know if a function result is mutable or not, it's the module that will compute
this information and update the AST function nodes accordingly
*/

use crate::error::TypingError;

use crate::references::References;
use crate::typer;
use vif_objects::ast::Expr;
use vif_objects::ast::ExprBody;
use vif_objects::ast::Function;
use vif_objects::ast::Stmt;
use vif_objects::ast::Value;

pub fn check_mutability(mut function: Function) -> Result<Function, TypingError> {
    let mut references = References::new();
    typer::add_missing_typing(&mut function, &mut references)?;

    check_function(&function)?;
    Ok(function)
}

fn check_function(function: &Function) -> Result<(), TypingError> {
    check_statements(&function.body)
}

fn check_statements(stmts: &Vec<Stmt>) -> Result<(), TypingError> {
    for stmt in stmts.iter() {
        check_statement(stmt)?;
    }
    Ok(())
}

fn check_statement(stmt: &Stmt) -> Result<(), TypingError> {
    match stmt {
        Stmt::Var(v) => {
            check_expression(&v.value)?;

            if v.typing.mutable && !v.value.typing.mutable {
                return Err(TypingError::Mutability(format!(
                    "Cannot set non mutable expression to mutable variable {}",
                    v.name
                )));
            }

            Ok(())
        }
        Stmt::Function(f) => check_function(f),
        Stmt::Expression(e) => check_expression(e),
        Stmt::Block(s) => check_statements(s),
        Stmt::Condition(c) => {
            check_expression(&c.expr)?;
            check_statement(&c.r#then)?;
            if c.r#else.is_some() {
                check_statement(c.r#else.as_deref().unwrap())?;
            };
            Ok(())
        }
        Stmt::While(w) => {
            check_expression(&r#w.condition)?;
            check_statement(&r#w.body)?;
            Ok(())
        }
        Stmt::Return(r) => check_expression(&r#r.value),
        Stmt::Assert(a) => check_expression(&r#a.value),
    }
}

fn check_expression(expr: &Expr) -> Result<(), TypingError> {
    match &expr.body {
        ExprBody::Value(Value::Variable(v)) => {}
        ExprBody::Call(c) => {
            check_expression(&c.callee)?;

            for arg in c.arguments.iter() {
                check_expression(arg)?;
            }

            if c.callee.typing.callable.is_none() {
                // probably builtin, TO FIX
                return Ok(());
            }

            if c.arguments.len()
                != c.callee
                    .typing
                    .callable
                    .as_ref()
                    .unwrap()
                    .signature
                    .parameters
                    .len()
            {
                return Err(TypingError::Mutability(format!(
                    "Wrong arguments numbers for function {}, expected {:?} got {:?}",
                    c.callee,
                    c.callee
                        .typing
                        .callable
                        .as_ref()
                        .unwrap()
                        .signature
                        .parameters,
                    c.arguments
                )));
            }

            for (arg, param_mutable) in c.arguments.iter().zip(
                c.callee
                    .typing
                    .callable
                    .as_ref()
                    .unwrap()
                    .signature
                    .parameters
                    .iter(),
            ) {
                if *param_mutable && !arg.typing.mutable {
                    return Err(TypingError::Mutability(format!(
                        "Cannot pass {} argument (non mutable) to a mutable parameter",
                        arg.body
                    )));
                }
            }
        }
        ExprBody::Binary(b) => {
            check_expression(&b.left)?;
            check_expression(&b.right)?;
            if b.left.typing.callable != b.right.typing.callable {
                return Err(TypingError::Signature(format!(
                    "{} and {} don't have the same signature: {:?} {:?}",
                    b.left, b.right, b.left.typing.callable, b.right.typing.callable
                )));
            }
        }
        ExprBody::Unary(u) => {
            check_expression(&u.right)?;
        }
        ExprBody::Grouping(g) => {
            check_expression(&g.expr)?;
        }
        ExprBody::Assign(a) => {
            if !expr.typing.mutable {
                return Err(TypingError::Mutability(format!(
                    "Cannot assign a value to {} (non mutable variable)",
                    a.name
                )));
            }

            check_expression(&a.value)?;
        }
        ExprBody::Logical(l) => {
            check_expression(&l.left)?;
            check_expression(&l.right)?;

            if l.left.typing.callable != l.right.typing.callable {
                return Err(TypingError::Signature(format!(
                    "{} and {} don't have the same signature",
                    l.left, l.right
                )));
            }
        }
        ExprBody::LoopKeyword(_) => (),
        ExprBody::Value(_) => (),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_mutability;
    use super::TypingError;
    use vif_ast::build_ast;

    #[test]
    fn simple_variable() {
        let string = "
            var i = 0
        "
        .to_owned();

        let ast = check_mutability(build_ast(string).unwrap()).unwrap();
        assert_eq!(ast.body.len(), 1);
    }

    #[test]
    fn cannot_override_non_mutable() {
        let string = "
            var i = 0
            i = 2
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_err());
        let err_msg = match result.unwrap_err() {
            TypingError::Mutability(s) => s,
            TypingError::Signature(s) => s,
        };
        assert_eq!(err_msg, "Cannot assign a value to i (non mutable variable)");
    }

    #[test]
    fn can_override_mutable() {
        let string = "
            var mut i = 0
            i = 2
        "
        .to_owned();

        let ast = check_mutability(build_ast(string).unwrap()).unwrap();
        assert_eq!(ast.body.len(), 2);
    }

    #[test]
    fn cannot_assign_const_to_mut() {
        let string = "
            var i = 0
            var mut j = i
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_err());
        let err_msg = match result.unwrap_err() {
            TypingError::Mutability(s) => s,
            TypingError::Signature(s) => s,
        };
        assert_eq!(
            err_msg,
            "Cannot set non mutable expression to mutable variable j"
        );
    }

    #[test]
    fn can_assign_mut_to_const() {
        let string = "
            var mut i = 0
            var j = i
        "
        .to_owned();

        let ast = check_mutability(build_ast(string).unwrap()).unwrap();
        assert_eq!(ast.body.len(), 2);
    }

    #[test]
    fn can_use_function_with_simple_values() {
        let string = "
            def coucou(a, mut b):
                return a + b

            coucou(1, 2)
        "
        .to_owned();

        let ast = check_mutability(build_ast(string).unwrap()).unwrap();
        assert_eq!(ast.body.len(), 2);
    }

    #[test]
    fn can_use_function_with_variables() {
        let string = "
            def coucou(a, mut b):
                return a + b

            var i = 1
            var mut j = 2
            coucou(i, j)
        "
        .to_owned();

        let ast = check_mutability(build_ast(string).unwrap()).unwrap();
        assert_eq!(ast.body.len(), 4);
    }

    #[test]
    fn cannot_use_function_with_const_on_mut() {
        let string = "
            def coucou(a, mut b):
                return a + b

            var i = 1
            var mut j = 2
            coucou(j, i)
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_err());
        let err_msg = match result.unwrap_err() {
            TypingError::Mutability(s) => s,
            TypingError::Signature(s) => s,
        };
        assert_eq!(
            err_msg,
            "Cannot pass Value[var[i]] argument (non mutable) to a mutable parameter"
        );
    }

    #[test]
    fn cannot_use_const_value_to_mut_variable() {
        let string = "
            def coucou(a):
                return a

            var i = 1
            var mut k = coucou(i)
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_err());
        let err_msg = match result.unwrap_err() {
            TypingError::Mutability(s) => s,
            TypingError::Signature(s) => s,
        };
        assert_eq!(
            err_msg,
            "Cannot set non mutable expression to mutable variable k"
        );
    }

    #[test]
    fn callable_variable_are_working_well() {
        let string = "
            def coucou(mut a):
                return a

            var i = coucou
            i(1)
        "
        .to_owned();

        let ast = check_mutability(build_ast(string).unwrap()).unwrap();
        assert_eq!(ast.body.len(), 3);
    }

    #[test]
    fn callable_variable_fail_passed_const_instead_of_mut() {
        let string = "
            def coucou(mut a):
                return a

            var i = coucou
            var j = 2
            i(j)
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_err());
        let err_msg = match result.unwrap_err() {
            TypingError::Mutability(s) => s,
            TypingError::Signature(s) => s,
        };
        assert_eq!(
            err_msg,
            "Cannot pass Value[var[j]] argument (non mutable) to a mutable parameter"
        );
    }

    #[test]
    fn callable_returned_by_function_are_ok() {
        let string = "
            def coucou(mut a):
                return a

            def test(mut p):
                return coucou

            test(2)(2)
        "
        .to_owned();

        let ast = check_mutability(build_ast(string).unwrap()).unwrap();
        assert_eq!(ast.body.len(), 3);
    }

    #[test]
    fn callable_returned_by_function_fail_when_passed_const() {
        let string = "
            def coucou(mut a):
                return a

            def test(mut p):
                return coucou

            var i = 2
            test(i)(i)
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_err());
        let err_msg = match result.unwrap_err() {
            TypingError::Mutability(s) => s,
            TypingError::Signature(s) => s,
        };
        assert_eq!(
            err_msg,
            "Cannot pass Value[var[i]] argument (non mutable) to a mutable parameter"
        );
    }

    #[test]
    fn callable_returning_a_callback_ok() {
        let string = "
            def coucou(mut a):
                def inside():
                    return 3
                return inside

            var callback = coucou(1)
            assert callback() == 3
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn callable_returning_a_callback_fail() {
        let string = "
            def coucou(mut a):
                def inside():
                    return 3
                return inside

            var callback = coucou(1)
            assert callback(2) == 3
        "
        .to_owned();

        let result = check_mutability(build_ast(string).unwrap());
        assert!(result.is_err());
        let err_msg = match result.unwrap_err() {
            TypingError::Mutability(s) => s,
            TypingError::Signature(s) => s,
        };
        assert!(err_msg.starts_with("Wrong arguments numbers for function"));
    }
}
