use vif_ast as ast;
use vif_ast::TypeAnnotation;

use crate::error::DifferentSignatureBetweenReturns;
use crate::error::IncompatibleTypes;
use crate::error::TypingError;

use crate::error::UnknownVariable;
use crate::objects::Assert;
use crate::objects::Assign;
use crate::objects::Binary;
use crate::objects::Call;
use crate::objects::Callable;
use crate::objects::CallableParameter;
use crate::objects::Condition;
use crate::objects::Entrypoint;
use crate::objects::Expr;
use crate::objects::ExprBody;
use crate::objects::Function;
use crate::objects::FunctionParameter;
use crate::objects::Logical;
use crate::objects::Return;
use crate::objects::Stmt;
use crate::objects::Type;
use crate::objects::Unary;
use crate::objects::UnaryOperator;
use crate::objects::Value;
use crate::objects::Variable;
use crate::objects::While;

use crate::references::FunctionReference;
use crate::references::Reference;
use crate::references::References;
use crate::references::VariableReference;

use crate::type_merger::TypeMerger;

pub struct BottomUpTyper<M>
where
    M: TypeMerger,
{
    type_merger: M,
}

impl<M> BottomUpTyper<M>
where
    M: TypeMerger,
{
    pub fn new(type_merger: M) -> Self {
        BottomUpTyper { type_merger }
    }

    pub fn run(entrypoint: &ast::Entrypoint, type_merger: M) -> Result<Entrypoint, TypingError> {
        let typer = Self::new(type_merger);
        let mut references = References::new();

        let stmts = entrypoint
            .body
            .iter()
            .map(|stmt| typer.visit_statement(stmt, &mut references))
            .collect::<Result<Vec<Stmt>, TypingError>>()?;

        Ok(Entrypoint { body: stmts })
    }

    pub fn visit_function(
        &self,
        function: &ast::Function,
        references: &mut References,
    ) -> Result<Function, TypingError> {
        let index = references.len();

        references.push(Reference::Function(FunctionReference {
            name: function.name.clone(),
            output: Type::Unknown,
        }));

        let params = function
            .params
            .iter()
            .map(|param| {
                let typing = param
                    .annotation
                    .as_ref()
                    .map_or(Type::Unknown, |a| Type::from_annotation(a));

                references.push(Reference::Variable(VariableReference::new(
                    param.name.clone(),
                    typing.clone(),
                    param.mutable,
                )));
                FunctionParameter::new(param.name.clone(), param.mutable, typing.clone())
            })
            .collect();

        let stmts = function
            .body
            .iter()
            .map(|stmt| self.visit_statement(stmt, references))
            .collect::<Result<Vec<Stmt>, TypingError>>()?;

        let mut typed_function = Function::new(function.name.clone(), params, stmts, Type::Unknown);

        self.update_function_typing(&mut typed_function)?;

        references.truncate(index);

        references.push(Reference::new_function(
            typed_function.name.clone(),
            typed_function.output.clone(),
        ));

        Ok(typed_function)
    }

    fn update_function_typing(&self, function: &mut Function) -> Result<(), TypingError> {
        let returns = function
            .body
            .iter()
            .map(|b| b.get_all_returns())
            .flatten()
            .filter(|r| r.value.typing != Type::Unknown)
            .collect::<Vec<&Return>>();

        // let signature =
        //     Signature::new_with_params(function.params.iter().map(|p| p.typing.clone()).collect());

        // let param_names = function
        //     .params
        //     .iter()
        //     .map(|p| p.name.as_str())
        //     .collect::<Vec<&str>>();

        // let return_pointers = returns.iter().any(|r| {
        //     if !r.value.typing.mutable {
        //         return false; // TODO: maybe I should just never returns a pointer
        //     }
        //     for name in get_identifier_names(&r.value) {
        //         if param_names.contains(&name.as_str()) {
        //             println!("YES {} {}", function.name, name);
        //             return true;
        //         }
        //     }
        //     false
        // });

        if returns.is_empty() {
            function.output = Type::None;
        } else {
            function.output = returns[0].value.typing.clone();
        };

        for return_stmt in returns.iter() {
            if return_stmt.value.typing.get_concrete_type() != function.output.get_concrete_type() {
                return Err(DifferentSignatureBetweenReturns::new(
                    function.name.clone(),
                    return_stmt.value.typing.clone(),
                    function.output.clone(),
                    return_stmt.value.span.clone(),
                ));
            }
        }

        Ok(())
    }

    fn visit_statement<'a>(
        &self,
        stmt: &ast::Stmt,
        references: &mut References,
    ) -> Result<Stmt, TypingError> {
        Ok(match stmt {
            ast::Stmt::Expression(expr) => {
                Stmt::Expression(self.visit_expression(expr, references)?)
            }
            ast::Stmt::Block(block) => {
                let stmts = block
                    .iter()
                    .map(|stmt| self.visit_statement(stmt, references))
                    .collect::<Result<Vec<Stmt>, TypingError>>()?;

                Stmt::Block(stmts)
            }
            ast::Stmt::Condition(cond) => {
                let expr = self.visit_expression(&cond.expr, references)?;
                let then = self.visit_statement(&cond.then, references)?;
                let mut r#else = None;

                if let Some(stmt_else) = &cond.r#else {
                    r#else.replace(Box::new(self.visit_statement(&stmt_else, references)?));
                };

                Stmt::Condition(Condition::new(expr, Box::new(then), r#else))
            }
            ast::Stmt::Return(ret) => {
                let expr = self.visit_expression(&ret.value, references)?;
                let typing = expr.typing.clone();
                Stmt::Return(Return::new(expr, typing))
            }
            ast::Stmt::Assert(assert) => {
                let expr = self.visit_expression(&assert.value, references)?;
                Stmt::Assert(Assert::new(expr))
            }
            ast::Stmt::While(block) => {
                let expr = self.visit_expression(&block.condition, references)?;
                let body = self.visit_statement(&block.body, references)?;
                Stmt::While(While::new(expr, Box::new(body)))
            }
            ast::Stmt::Var(v) => {
                let expr = self.visit_expression(&v.value, references)?;
                let typing = expr.typing.clone();

                if let Some(annotation) = &v.annotation {
                    match (annotation, &typing) {
                        (TypeAnnotation::Int, Type::Int) => (),
                        (TypeAnnotation::Float, Type::Float) => (),
                        (TypeAnnotation::String, Type::String) => (),
                        (TypeAnnotation::Bool, Type::Bool) => (),
                        (TypeAnnotation::None, Type::None) => (),
                        (ann, typ) => {
                            return Err(IncompatibleTypes::new(
                                ann.to_string(),
                                typ.to_string(),
                                expr.span.clone(),
                            ))
                        }
                    }
                }
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

                references.push(Reference::new_variable(
                    v.name.clone(),
                    typing.clone(),
                    v.mutable,
                ));

                Stmt::Var(Variable::new(v.name.clone(), expr, v.mutable, typing))
            }
            ast::Stmt::Function(f) => Stmt::Function(self.visit_function(f, references)?),
        })
    }

    fn visit_expression(
        &self,
        expr: &ast::Expr,
        references: &mut References,
    ) -> Result<Expr, TypingError> {
        Ok(match &expr.body {
            ast::ExprBody::Value(ast::Value::Variable(v)) => {
                if let Some(typing) = references.get_typing(v.as_str()) {
                    Expr::new(
                        ExprBody::Value(ast::Value::Variable(v.clone())),
                        expr.span.clone(),
                        typing,
                    )
                } else {
                    match v.as_str() {
                        "print" => Expr::new(
                            ExprBody::Value(ast::Value::Variable("print".to_owned())),
                            expr.span.clone(),
                            Type::Callable(Callable::new_infinite(Box::new(Type::None))),
                        ),
                        // "get_time" =>
                        // "sleep" =>
                        _ => return Err(UnknownVariable::new(v.clone(), expr.span.clone())),
                    }
                }
            }
            ast::ExprBody::Value(v) => Expr::new(
                ExprBody::Value(v.clone()),
                expr.span.clone(),
                match v {
                    ast::Value::String(_) => Type::String,
                    ast::Value::Integer(_) => Type::Int,
                    ast::Value::Float(_) => Type::Float,
                    ast::Value::True => Type::Bool,
                    ast::Value::False => Type::Bool,
                    ast::Value::None => Type::None,
                    ast::Value::Variable(_) => unreachable!(),
                },
            ),
            ast::ExprBody::Binary(binary) => {
                let left = self.visit_expression(&binary.left, references)?;
                let right = self.visit_expression(&binary.right, references)?;
                let typing = self
                    .type_merger
                    .merge(&left.typing, &right.typing)
                    .ok_or_else(|| {
                        IncompatibleTypes::new(
                            left.typing.as_string(),
                            right.typing.as_string(),
                            expr.span.clone(),
                        )
                    })?;

                Expr::new(
                    ExprBody::Binary(Binary::new(
                        Box::new(left),
                        binary.operator.clone(),
                        Box::new(right),
                    )),
                    expr.span.clone(),
                    typing,
                )
            }
            ast::ExprBody::Unary(unary) => {
                let right = self.visit_expression(&unary.right, references)?;
                let typing = match unary.operator {
                    UnaryOperator::Minus => right.typing.clone(),
                    UnaryOperator::Not => Type::Bool,
                };

                Expr::new(
                    ExprBody::Unary(Unary::new(unary.operator.clone(), Box::new(right))),
                    expr.span.clone(),
                    typing,
                )
            }
            ast::ExprBody::Assign(assign) => {
                let value = self.visit_expression(&assign.value, references)?;
                let typing = value.typing.clone();
                Expr::new(
                    ExprBody::Assign(Assign::new(
                        assign.name.to_owned(),
                        Box::new(value),
                        typing.clone(),
                    )),
                    expr.span.clone(),
                    typing,
                )
            }
            ast::ExprBody::Logical(logical) => {
                let left = self.visit_expression(&logical.left, references)?;
                let right = self.visit_expression(&logical.right, references)?;

                let typing = self
                    .type_merger
                    .merge(&left.typing, &right.typing)
                    .ok_or_else(|| {
                        IncompatibleTypes::new(
                            left.typing.as_string(),
                            right.typing.as_string(),
                            expr.span.clone(),
                        )
                    })?;

                Expr::new(
                    ExprBody::Logical(Logical::new(
                        Box::new(left),
                        logical.operator.clone(),
                        Box::new(right),
                    )),
                    expr.span.clone(),
                    typing,
                )
            }
            ast::ExprBody::Call(call) => {
                let callee = self.visit_expression(&call.callee, references)?;
                let arguments = call
                    .arguments
                    .iter()
                    .map(|arg| self.visit_expression(arg, references).map(|e| Box::new(e)))
                    .collect::<Result<Vec<Box<Expr>>, TypingError>>()?;
                let typing = callee.typing.clone();

                let expr = Expr::new(
                    ExprBody::Call(Call::new(Box::new(callee), arguments)),
                    expr.span.clone(),
                    typing.clone(),
                );

                let callable_names = get_identifier_names(&expr);

                // check function parameters typing
                for callable_name in callable_names.iter() {
                    if let Some(function_reference) =
                        references.get_function_typing_ref(callable_name)
                    {
                        match (&function_reference, &expr.typing) {
                            (Type::Callable(function_callable), Type::Callable(callable)) => {
                                match (&function_callable.parameters, &callable.parameters) {
                                    (
                                        CallableParameter::Parameters(function_params),
                                        CallableParameter::Parameters(params),
                                    ) => {
                                        for (param, arg) in
                                            function_params.iter().zip(params.iter())
                                        {
                                            if param.typing != arg.typing {
                                                return Err(IncompatibleTypes::new(
                                                    param.typing.as_string(),
                                                    arg.typing.as_string(),
                                                    expr.span.clone(),
                                                ));
                                            }
                                        }
                                    }
                                    (CallableParameter::Infinite, CallableParameter::Infinite) => {
                                        continue
                                    }
                                    _ => panic!("what is this"),
                                }
                            }
                            _ => continue,
                        }
                    }
                }

                // update function parameters typing if it's them being called
                // for param in params.iter_mut() {
                //     if callable_names.contains(&param.name) {
                //         param.typing = call.callee.typing.clone();
                //     }
                // }

                expr
            }
            ast::ExprBody::LoopKeyword(v) => Expr::new(
                ExprBody::LoopKeyword(v.clone()),
                expr.span.clone(),
                Type::KeyWord,
            ),
        })
    }
}

fn get_identifier_names(expr: &Expr) -> Vec<String> {
    match &expr.body {
        ExprBody::Value(Value::Variable(v)) => {
            vec![v.to_owned()]
        }
        ExprBody::Unary(unary) => get_identifier_names(&unary.right),
        ExprBody::Logical(logical) => {
            let mut res = get_identifier_names(&logical.left);
            res.extend(get_identifier_names(&logical.right));
            res
        }
        ExprBody::Call(c) => get_identifier_names(&c.callee),
        ExprBody::Binary(_) => Vec::new(),
        ExprBody::Assign(_) => Vec::new(),
        ExprBody::LoopKeyword(_) => Vec::new(),
        ExprBody::Value(_) => Vec::new(),
    }
}
