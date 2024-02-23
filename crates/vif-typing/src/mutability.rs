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

#[derive(Debug, Clone)]
struct VariableReference {
    name: String,
    mutable: bool,
}

impl std::cmp::PartialEq for VariableReference {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.mutable == other.mutable
    }
}

impl VariableReference {
    fn new(name: String, mutable: bool) -> Self {
        Self { name, mutable }
    }
}

struct FunctionReference {
    name: String,
    mutable: bool,
    parameters: Vec<VariableReference>,
}

enum Reference {
    Variable(VariableReference),
    Function(FunctionReference),
}

impl Reference {
    fn new_variable(name: String, mutable: bool) -> Self {
        Self::Variable(VariableReference { name, mutable })
    }

    fn new_function(name: String, mutable: bool, parameters: Vec<VariableReference>) -> Self {
        Self::Function(FunctionReference {
            name,
            mutable,
            parameters,
        })
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(v) => write!(f, "var {}", v.name),
            Self::Function(v) => write!(f, "func {} {:?}", v.name, v.parameters),
        }
    }
}

impl std::fmt::Debug for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(v) => write!(f, "var {}", v.name),
            Self::Function(v) => write!(f, "func {} {:?}", v.name, v.parameters),
        }
    }
}

struct References {
    references: Vec<Reference>,
}

impl std::fmt::Display for References {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.references
                .iter()
                .map(|r| format!("{r}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl References {
    fn new() -> Self {
        References {
            references: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.references.len()
    }

    fn truncate(&mut self, value: usize) {
        self.references.truncate(value)
    }

    fn push(&mut self, value: Reference) {
        self.references.push(value)
    }

    fn contain_mutable_reference(&self, name: &str) -> bool {
        self.references
            .iter()
            .rev()
            .find(|r| match r {
                Reference::Variable(v) if v.name == name && v.mutable => true,
                Reference::Function(f) if f.name == name && f.mutable => true,
                _ => false,
            })
            .is_some()
    }

    fn get_function(&self, name: &str) -> Option<&FunctionReference> {
        self.references.iter().rev().find_map(|r| match r {
            Reference::Function(f) if f.name == name => Some(f),
            _ => None,
        })
    }
}

pub fn check_mutability(mut function: Function) -> Result<Function, TypingError> {
    let mut references = References::new();
    check_function(&mut function, &mut references)?;
    Ok(function)
}

fn check_function(function: &mut Function, references: &mut References) -> Result<(), TypingError> {
    let index = references.len();

    for param in function.params.iter() {
        if param.mutable {
            references.push(Reference::new_variable(param.name.clone(), true));
        };
    }

    check_statements(&mut function.body, references)?;

    references.truncate(index);

    function.mutable = function
        .body
        .iter()
        .filter_map(|s| match s {
            Stmt::Return(r) => Some(r),
            _ => None,
        })
        .all(|r| r.value.mutable);

    let parameters = function
        .params
        .iter()
        .map(|p| VariableReference::new(p.name.clone(), p.mutable))
        .collect::<Vec<VariableReference>>();

    references.push(Reference::new_function(
        function.name.clone(),
        function.mutable,
        parameters,
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

            if v.mutable && !v.value.mutable {
                return Err(TypingError::Mutability(format!(
                    "Cannot set non mutable expression to mutable variable {}",
                    v.name
                )));
            }

            if v.mutable {
                references.push(Reference::new_variable(v.name.clone(), true));
            }

            if let Some(params) = get_function_parameters(&v.value, references) {
                references.push(Reference::new_function(
                    v.name.clone(),
                    v.value.mutable,
                    params.clone(),
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
                expr.mutable = true;
            }
        }
        ExprBody::Call(c) => {
            check_expression(&mut c.callee, references)?;
            expr.mutable = c.callee.mutable;
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
                if param.mutable && !arg.mutable {
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
            expr.mutable = true;
        }
        ExprBody::Unary(u) => {
            check_expression(&mut u.right, references)?;
            expr.mutable = u.right.mutable;
        }
        ExprBody::Grouping(g) => {
            check_expression(&mut g.expr, references)?;
            expr.mutable = g.expr.mutable;
        }
        ExprBody::Assign(a) => {
            if !references.contain_mutable_reference(&a.name) {
                return Err(TypingError::Mutability(format!(
                    "Cannot assign a value to {} (non mutable variable)",
                    a.name
                )));
            }
            check_expression(&mut a.value, references)?;
            expr.mutable = a.value.mutable;
        }
        ExprBody::Logical(l) => {
            check_expression(&mut l.left, references)?;
            check_expression(&mut l.right, references)?;
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
