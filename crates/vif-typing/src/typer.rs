use crate::error::DifferentSignatureBetweenReturns;
use crate::error::FunctionReturnsDifferentTypes;
use crate::error::IncompatibleTypes;
use crate::error::TypingError;
use crate::references::FunctionReference;
use crate::references::Reference;
use crate::references::References;
use crate::references::VariableReference;
use vif_objects::ast::Callable;
use vif_objects::ast::Expr;
use vif_objects::ast::ExprBody;
use vif_objects::ast::Function;
use vif_objects::ast::FunctionParameter;
use vif_objects::ast::LogicalOperator;
use vif_objects::ast::Return;
use vif_objects::ast::Signature;
use vif_objects::ast::Stmt;
use vif_objects::ast::Type;
use vif_objects::ast::Typing;
use vif_objects::ast::Value;

pub fn add_missing_typing<'a>(
    function: &mut Function,
    references: &mut References,
) -> Result<(), TypingError> {
    let index = references.len();

    references.push(Reference::Function(FunctionReference {
        name: function.name.clone(),
        typing: function.typing.clone(),
    }));

    for param in function.params.iter() {
        references.push(Reference::Variable(VariableReference::new(
            param.name.clone(),
            param.typing.clone(),
        )));
    }

    for stmt in function.body.iter_mut() {
        visit_statement(&mut function.params, stmt, references)?;
    }

    update_function_typing(function)?;

    references.truncate(index);

    references.push(Reference::new_function(
        function.name.clone(),
        function.typing.clone(),
    ));

    Ok(())
}

fn update_function_typing(function: &mut Function) -> Result<(), TypingError> {
    let returns = function
        .body
        .iter()
        .filter_map(|s| match s {
            Stmt::Return(r) => Some(r),
            _ => None,
        })
        .collect::<Vec<&Return>>();

    let signature =
        Signature::new_with_params(function.params.iter().map(|p| p.typing.clone()).collect());

    let callable = if returns.is_empty() {
        Box::new(Callable::new(signature, Typing::new(false, Type::None)))
    } else {
        Box::new(Callable::new(signature, returns[0].value.typing.clone()))
    };

    for return_stmt in returns.iter() {
        if return_stmt.value.typing.r#type != callable.output.r#type {
            return Err(DifferentSignatureBetweenReturns::new(
                function.name.clone(),
                return_stmt.value.typing.clone(),
                function.typing.clone(),
                return_stmt.value.span.clone(),
            ));
        }
    }

    function.typing = Typing::new(
        returns.iter().all(|r| r.value.typing.mutable),
        Type::Callable(callable),
    );

    Ok(())
}

fn visit_statement<'a>(
    params: &mut Vec<FunctionParameter>,
    stmt: &mut Stmt,
    references: &mut References,
) -> Result<(), TypingError> {
    match stmt {
        Stmt::Expression(expr) => visit_expression(params, expr, references)?,
        Stmt::Block(block) => {
            for stmt in block.iter_mut() {
                visit_statement(params, stmt, references)?;
            }
        }
        Stmt::Condition(cond) => {
            visit_expression(params, &mut cond.expr, references)?;
            visit_statement(params, &mut cond.then, references)?;
            if let Some(stmt_else) = &mut cond.r#else {
                visit_statement(params, stmt_else, references)?;
            }
        }
        Stmt::Return(ret) => visit_expression(params, &mut ret.value, references)?,
        Stmt::Assert(assert) => visit_expression(params, &mut assert.value, references)?,
        Stmt::While(block) => {
            visit_expression(params, &mut block.condition, references)?;
            visit_statement(params, &mut block.body, references)?;
        }
        Stmt::Var(v) => {
            visit_expression(params, &mut v.value, references)?;
            v.typing.r#type = v.value.typing.r#type.clone();

            // should not be needed as we get identifier typing from the call above

            // // we might assign a variable to another variable
            // // var a = b or c
            // let names = get_identifier_names(&v.value);

            // for name in names.iter() {
            //     if let Some(t) = references.get_typing(name) {
            //         if let Some(callable) = t.callable {
            //             match v.value.body {
            //                 ExprBody::Call(_) => v.typing.callable = callable.output.callable,
            //                 _ => v.typing.callable = Some(callable),
            //             };
            //         }
            //         break;
            //     }
            // }

            references.push(Reference::new_variable(v.name.clone(), v.typing.clone()))
        }
        Stmt::Function(f) => {
            add_missing_typing(f, references)?;
        }
    };

    Ok(())
}

fn visit_expression<'a>(
    params: &mut Vec<FunctionParameter>,
    expr: &mut Expr,
    references: &mut References,
) -> Result<(), TypingError> {
    match &mut expr.body {
        ExprBody::Binary(binary) => {
            visit_expression(params, &mut binary.left, references)?;
            visit_expression(params, &mut binary.right, references)?;

            expr.typing.r#type = binary
                .left
                .typing
                .r#type
                .hard_merge(&binary.right.typing.r#type)
                .map_err(|_| {
                    IncompatibleTypes::new(
                        binary.left.typing.r#type.as_string(),
                        binary.right.typing.r#type.as_string(),
                        expr.span.clone(),
                    )
                })?;
            expr.typing.mutable = true;
        }
        ExprBody::Unary(unary) => {
            visit_expression(params, &mut unary.right, references)?;
            expr.typing.r#type = unary.right.typing.r#type.clone();
            expr.typing = unary.right.typing.clone();
        }
        ExprBody::Grouping(grouping) => {
            visit_expression(params, &mut grouping.expr, references)?;
            expr.typing.r#type = grouping.expr.typing.r#type.clone();
            expr.typing = grouping.expr.typing.clone();
        }
        ExprBody::Assign(assign) => {
            visit_expression(params, &mut assign.value, references)?;

            if let Some(t) = references.get_typing(&assign.name) {
                expr.typing.r#type = t.r#type.clone();
                expr.typing.mutable = t.mutable;
            }
            // TODO: should probably override the variable we have in references here
        }
        ExprBody::Logical(logical) => {
            visit_expression(params, &mut logical.left, references)?;
            visit_expression(params, &mut logical.right, references)?;

            match logical.operator {
                LogicalOperator::And => {
                    expr.typing.r#type = Type::Bool;
                    expr.typing.mutable = true;
                }
                LogicalOperator::Or => {
                    expr.typing.r#type = logical
                        .left
                        .typing
                        .r#type
                        .hard_merge(&logical.right.typing.r#type)
                        .map_err(|_| {
                            IncompatibleTypes::new(
                                logical.left.typing.r#type.as_string(),
                                logical.right.typing.r#type.as_string(),
                                expr.span.clone(),
                            )
                        })?;
                    expr.typing.mutable =
                        logical.left.typing.mutable && logical.right.typing.mutable;
                }
            }
        }
        ExprBody::Call(call) => {
            visit_expression(params, &mut call.callee, references)?;
            for arg in call.arguments.iter_mut() {
                visit_expression(params, arg, references)?;
            }

            expr.typing.r#type = call.callee.typing.r#type.clone();
            expr.typing.mutable = call.callee.typing.mutable;
            let callable_names = get_identifier_names(&call.callee);

            // // add callee typing
            // for name in callable_names.iter() {
            //     if let Some(typing) = references.get_typing(name) {
            //         call.callee.typing.callable = typing.callable;
            //         break;
            //     }
            // }

            // if call.callee.typing.callable.is_none() {
            // panic!(
            //     "Oh bah non alors: {} {:?} and {}",
            //     call.callee, callable_names, references
            // );
            // }

            // update function parameters typing if it's them being called
            for param in params.iter_mut() {
                if callable_names.contains(&param.name) {
                    param.typing = call.callee.typing.clone();
                }
            }
        }
        ExprBody::Value(Value::Variable(v)) => {
            if let Some(typing) = references.get_typing(v.as_str()) {
                expr.typing = typing;
            } else {
                match v.as_str() {
                    "print" => {
                        expr.typing = Typing::new(
                            false,
                            Type::Callable(Box::new(Callable::new(
                                Signature::new_with_infinite(),
                                Typing::new(true, Type::None),
                            ))),
                        )
                    }
                    // "get_time" =>
                    // "sleep" =>
                    _ => panic!("Unknown variable ? {}", v),
                }
            }
        }
        ExprBody::Value(_) => expr.typing.mutable = true,
        ExprBody::LoopKeyword(_) => expr.typing.mutable = false,
    };
    Ok(())
}

fn get_identifier_names(expr: &Expr) -> Vec<String> {
    match &expr.body {
        ExprBody::Binary(binary) => {
            let mut res = get_identifier_names(&binary.left);
            res.extend(get_identifier_names(&binary.right));
            res
        }
        ExprBody::Unary(unary) => get_identifier_names(&unary.right),
        ExprBody::Grouping(grouping) => get_identifier_names(&grouping.expr),
        ExprBody::Logical(logical) => {
            let mut res = get_identifier_names(&logical.left);
            res.extend(get_identifier_names(&logical.right));
            res
        }
        ExprBody::Value(Value::Variable(v)) => {
            vec![v.to_owned()]
        }
        ExprBody::Call(c) => get_identifier_names(&c.callee),
        _ => Vec::new(),
    }
}
