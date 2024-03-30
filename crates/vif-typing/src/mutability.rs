/*
This module checks that mutability is coherent through the whole program.
If a variable is declared as mutable, the module will ensure that only mutable values can be assigned it.
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

use crate::references::Reference;
use crate::references::References;
use crate::references::VariableReference;
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
    let mut references = References::new();
    check_function(&mut function, &mut references)?;
    Ok(function)
}

fn check_function(function: &mut Function, references: &mut References) -> Result<(), TypingError> {
    let index = references.len();

    for param in function.params.iter() {
        if param.typing.mutable {
            references.push(Reference::new_variable(
                param.name.clone(),
                param.typing.clone(),
            ));
        };
    }

    check_statements(&mut function.body, references)?;

    references.truncate(index);

    function.typing.mutable = function
        .body
        .iter()
        .filter_map(|s| match s {
            Stmt::Return(r) => Some(r),
            _ => None,
        })
        .all(|r| r.value.typing.mutable);

    let parameters = function
        .params
        .iter()
        .map(|p| VariableReference::new(p.name.clone(), p.typing.clone()))
        .collect::<Vec<VariableReference>>();

    references.push(Reference::new_function(
        function.name.clone(),
        parameters,
        function.typing.clone(),
    ));

    Ok(())
}

fn check_statements(stmts: &mut Vec<Stmt>, references: &mut References) -> Result<(), TypingError> {
    for stmt in stmts.iter_mut() {
        check_statement(stmt, references)?;
    }
    Ok(())
}

fn check_statement(stmt: &mut Stmt, references: &mut References) -> Result<(), TypingError> {
    match stmt {
        Stmt::Var(v) => {
            check_expression(&mut v.value, references)?;

            if v.typing.mutable && !v.value.typing.mutable {
                return Err(TypingError::Mutability(format!(
                    "Cannot set non mutable expression to mutable variable {}",
                    v.name
                )));
            }

            if v.typing.mutable {
                references.push(Reference::new_variable(v.name.clone(), v.typing.clone()));
            }

            if let Some(params) = get_function_parameters(&v.value, references) {
                references.push(Reference::new_function(
                    v.name.clone(),
                    params.clone(),
                    v.value.typing.clone(),
                ));
            }

            Ok(())
        }
        Stmt::Function(f) => check_function(f, references),
        Stmt::Expression(e) => check_expression(e, references),
        Stmt::Block(s) => check_statements(s, references),
        Stmt::Condition(c) => check_condition(c, references),
        Stmt::While(w) => check_while(w, references),
        Stmt::Return(r) => check_return(r, references),
        Stmt::Assert(a) => check_assert(a, references),
    }
}

fn check_expression(expr: &mut Expr, references: &mut References) -> Result<(), TypingError> {
    match &mut expr.body {
        ExprBody::Value(Value::Variable(v)) => {
            if references.contain_mutable_reference(v.as_str()) {
                expr.typing.mutable = true;
            }
        }
        ExprBody::Call(c) => {
            check_expression(&mut c.callee, references)?;
            expr.typing.mutable = c.callee.typing.mutable;
            for arg in c.arguments.iter_mut() {
                check_expression(arg, references)?;
            }

            let parameters = get_function_parameters(&c.callee, references);
            if parameters.is_none() {
                return Ok(());
            };

            let parameters = parameters.unwrap();

            if c.arguments.len() != parameters.len() {
                return Err(TypingError::Mutability(format!(
                    "Wrong arguments numbers for function {}",
                    c.callee
                )));
            }

            for (arg, param) in c.arguments.iter().zip(parameters.iter()) {
                if param.typing.mutable && !arg.typing.mutable {
                    return Err(TypingError::Mutability(format!(
                        "Cannot pass {} argument (non mutable) to {} parameter (mutable)",
                        arg.body, param.name
                    )));
                }
            }
        }
        ExprBody::Binary(b) => {
            check_expression(&mut b.left, references)?;
            check_expression(&mut b.right, references)?;
            expr.typing.mutable = true;
        }
        ExprBody::Unary(u) => {
            check_expression(&mut u.right, references)?;
            expr.typing.mutable = u.right.typing.mutable;
        }
        ExprBody::Grouping(g) => {
            check_expression(&mut g.expr, references)?;
            expr.typing.mutable = g.expr.typing.mutable;
        }
        ExprBody::Assign(a) => {
            if !references.contain_mutable_reference(&a.name) {
                return Err(TypingError::Mutability(format!(
                    "Cannot assign a value to {} (non mutable variable)",
                    a.name
                )));
            }
            check_expression(&mut a.value, references)?;
            expr.typing.mutable = a.value.typing.mutable;
        }
        ExprBody::Logical(l) => {
            check_expression(&mut l.left, references)?;
            check_expression(&mut l.right, references)?;
            match l.operator {
                LogicalOperator::And => {
                    expr.typing.mutable = true;
                }
                LogicalOperator::Or => {
                    expr.typing.mutable = l.left.typing.mutable && l.right.typing.mutable;
                }
            }
        }
        ExprBody::LoopKeyword(_) => (),
        ExprBody::Value(_) => (),
    };

    Ok(())
}

fn check_condition(cond: &mut Condition, references: &mut References) -> Result<(), TypingError> {
    check_expression(&mut cond.expr, references)?;
    check_statement(&mut cond.r#then, references)?;
    if cond.r#else.is_some() {
        check_statement(cond.r#else.as_deref_mut().unwrap(), references)?;
    };
    Ok(())
}

fn check_while(r#while: &mut While, references: &mut References) -> Result<(), TypingError> {
    check_expression(&mut r#while.condition, references)?;
    check_statement(&mut r#while.body, references)?;
    Ok(())
}

fn check_return(r#return: &mut Return, references: &mut References) -> Result<(), TypingError> {
    check_expression(&mut r#return.value, references)
}

fn check_assert(r#assert: &mut Assert, references: &mut References) -> Result<(), TypingError> {
    check_expression(&mut r#assert.value, references)
}

fn get_function_parameters<'a>(
    expr: &Expr,
    references: &'a References,
) -> Option<&'a Vec<VariableReference>> {
    match &expr.body {
        ExprBody::Value(Value::Variable(s)) => references
            .get_function(s.as_str())
            .and_then(|f| Some(&f.parameters)),
        ExprBody::Value(_) => None,
        ExprBody::Call(c) => {
            return get_function_parameters(&c.callee, references);
        }
        ExprBody::Unary(u) => get_function_parameters(&u.right, references),
        ExprBody::Binary(b) => {
            let right = get_function_parameters(&b.right, references);
            let left = get_function_parameters(&b.left, references);
            if right != left {}
            return left;
        }
        ExprBody::Grouping(g) => get_function_parameters(&g.expr, references),
        ExprBody::Assign(_) => None,
        ExprBody::LoopKeyword(_) => None,
        ExprBody::Logical(l) => {
            let right = get_function_parameters(&l.right, references);
            let left = get_function_parameters(&l.left, references);
            if right != left {}
            return left;
        }
    }
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
        };
        assert_eq!(
            err_msg,
            "Cannot pass Value[var[i]] argument (non mutable) to b parameter (mutable)"
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
        };
        assert_eq!(
            err_msg,
            "Cannot pass Value[var[j]] argument (non mutable) to a parameter (mutable)"
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
        };
        assert_eq!(
            err_msg,
            "Cannot pass Value[var[i]] argument (non mutable) to p parameter (mutable)"
        );
    }
}
