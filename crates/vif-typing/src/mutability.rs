/*
This module checks that mutability is coherent through the whole program.
If a variable is declared as mutable, we need to ensure that only mutable values can be assigned it.
It will also ensure that a variable not declared a mutable cannot be mutated.

First, what is a mutable value ?
It's basically a core type (int, string, bool...). A mutable value can loose its mutability, and can
never find it back later.

The mutability perperty of a value is transfered through variables or functions (parameters and returns).

In other words, everything is mutable until it lands into a variable that is not mutable.
From that moment, the value hold by the variable cannot be mutated.

The goal of that module is to ensure this statement is true at compile time.

It will check variables, functions, their paramters, their return values to assert if the mutability rules
are respected.

A lot of the complexcity resides in functions. Their parameters can be mutable or not. Their return values can
be mutable or not. Our mutability principle must be respected at every stage.

Example
```vif
def hello(s):
    return s

var const_var = "world"
var mut mut_var = hello(const_var) # should not be allowed
# we can't store in a mutable variable a result that is not mutable
```

For a function to be "considered" as mutable, all of the possible return values from that function
must be mutable. If not, the function return value is not considered as mutable.

The module will look for var declaration, assignment and calls to do its checks.
*/

use crate::error::DifferentSignatureBetweenFunction;
use crate::error::NonMutableArgumentToMutableParameter;
use crate::error::NonMutableArgumentToMutableVariable;
use crate::error::TypingError;
use crate::error::WrongArgumentNumberFunction;
use crate::references::References;
use crate::typer;
use vif_objects::ast::Expr;
use vif_objects::ast::ExprBody;
use vif_objects::ast::Function;
use vif_objects::ast::Stmt;
use vif_objects::ast::Value;

pub fn check_mutability(function: &mut Function) -> Result<(), TypingError> {
    let mut references = References::new();
    typer::add_missing_typing(function, &mut references)?;

    check_function(function)?;
    Ok(())
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
                return Err(NonMutableArgumentToMutableVariable::new(
                    v.name.clone(),
                    format!("{}", v.value),
                    v.value.span.clone(),
                ));
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
        ExprBody::Call(c) => {
            check_expression(&c.callee)?;

            for arg in c.arguments.iter() {
                check_expression(arg)?;
            }

            let callable = match &c.callee.typing.r#type {
                vif_objects::ast::Type::Callable(c) => c,
                _ => panic!("{} is not callable: {}", c.callee, c.callee.typing),
            };

            if let Some(signature_params) = callable.signature.get_params() {
                if signature_params.len() != c.arguments.len() {
                    return Err(WrongArgumentNumberFunction::new(
                        format!("{}", c.callee),
                        signature_params.len(),
                        c.arguments.len(),
                        c.callee.span.clone(),
                    ));
                }

                for (arg, param_typing) in c.arguments.iter().zip(signature_params.iter()) {
                    if param_typing.mutable && !arg.typing.mutable {
                        return Err(NonMutableArgumentToMutableParameter::new(
                            format!("{}", c.callee),
                            format!("{}", arg.body),
                            c.callee.span.clone(),
                        ));
                    }
                }
            }
        }
        ExprBody::Binary(b) => {
            check_expression(&b.left)?;
            check_expression(&b.right)?;
            if b.left.typing != b.right.typing {
                return Err(DifferentSignatureBetweenFunction::new(
                    format!("{}", b.left),
                    format!("{}", b.right),
                    b.left.typing.clone(),
                    b.right.typing.clone(),
                    b.right.span.clone(),
                ));
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
                return Err(NonMutableArgumentToMutableVariable::new(
                    a.name.clone(),
                    format!("{}", a.value),
                    a.value.span.clone(),
                ));
            }

            check_expression(&a.value)?;
        }
        ExprBody::Logical(l) => {
            check_expression(&l.left)?;
            check_expression(&l.right)?;

            if l.left.typing != l.right.typing {
                return Err(DifferentSignatureBetweenFunction::new(
                    format!("{}", l.left),
                    format!("{}", l.right),
                    l.left.typing.clone(),
                    l.right.typing.clone(),
                    l.right.span.clone(),
                ));
            }
        }
        ExprBody::LoopKeyword(_) => (),
        ExprBody::Value(Value::Variable(_)) => (),
        ExprBody::Value(_) => (),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_mutability;
    use vif_ast::build_ast;

    #[test]
    fn simple_variable() {
        let string = "
            var i = 0
        ";

        let mut ast = build_ast(string).unwrap();
        check_mutability(&mut ast).unwrap();
        assert_eq!(ast.body.len(), 1);
    }

    #[test]
    fn cannot_override_non_mutable() {
        let string = "
            var i = 0
            i = 2
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().format(string);
        assert_eq!(
            err_msg,
            "Line 3 -             i = 2\nCannot assign value Value[2] (non mutable) to mutable variable i"
        );
    }

    #[test]
    fn can_override_mutable() {
        let string = "
            var mut i = 0
            i = 2
        ";

        let mut ast = build_ast(string).unwrap();
        check_mutability(&mut ast).unwrap();
        assert_eq!(ast.body.len(), 2);
    }

    #[test]
    fn cannot_assign_const_to_mut() {
        let string = "
            var i = 0
            var mut j = i
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().format(string);
        assert_eq!(
            err_msg,
            "Line 3 -             var mut j = i\nCannot assign value Value[var[i]] (non mutable) to mutable variable j"
        );
    }

    #[test]
    fn can_assign_mut_to_const() {
        let string = "
            var mut i = 0
            var j = i
        ";

        let mut ast = build_ast(string).unwrap();
        check_mutability(&mut ast).unwrap();
        assert_eq!(ast.body.len(), 2);
    }

    #[test]
    fn can_use_function_with_simple_values() {
        let string = "
            def coucou(a, mut b):
                return a + b

            coucou(1, 2)
        ";

        let mut ast = build_ast(string).unwrap();
        check_mutability(&mut ast).unwrap();
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
        ";

        let mut ast = build_ast(string).unwrap();
        check_mutability(&mut ast).unwrap();

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
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().format(string);

        assert_eq!(
            err_msg,
            "Line 7 -             coucou(j, i)\nCannot pass Value[var[i]] argument (non mutable) to a mutable parameter"
        );
    }

    #[test]
    fn cannot_use_const_value_to_mut_variable() {
        let string = "
            def coucou(a):
                return a

            var i = 1
            var mut k = coucou(i)
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().format(string);

        assert_eq!(
            err_msg,
            "Line 6 -             var mut k = coucou(i)\nCannot assign value Call[Function[Value[var[coucou]]]] (non mutable) to mutable variable k"
        );
    }

    #[test]
    fn callable_variable_are_working_well() {
        let string = "
            def coucou(mut a):
                return a

            var i = coucou
            i(1)
        ";

        let mut ast = build_ast(string).unwrap();
        check_mutability(&mut ast).unwrap();

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
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().format(string);

        assert_eq!(
            err_msg,
            "Line 7 -             i(j)\nCannot pass Value[var[j]] argument (non mutable) to a mutable parameter"
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
        ";

        let mut ast = build_ast(string).unwrap();
        check_mutability(&mut ast).unwrap();

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
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().format(string);

        assert_eq!(
            err_msg,
            "Line 9 -             test(i)(i)\nCannot pass Value[var[i]] argument (non mutable) to a mutable parameter"
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
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);

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
        ";

        let mut ast = build_ast(string).unwrap();
        let result = check_mutability(&mut ast);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().format(string);

        assert_eq!(
            err_msg,
            "Line 8 -             assert callback(2) == 3\nWrong number of argument passed. Expected 0 but received 1"
        );
    }
}
