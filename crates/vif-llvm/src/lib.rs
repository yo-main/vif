mod builder;
mod compiler;
mod error;

use crate::error::CompilerError;
use compiler::Compiler;

use inkwell;
use vif_objects::function::Arity;
use vif_objects::function::Function;
use vif_objects::function::NativeFunction;
use vif_objects::function::NativeFunctionCallee;
use vif_objects::global::Global;
use vif_objects::op_code::OpCode;

pub fn compile(ast_function: &vif_objects::ast::Function) -> Result<(), CompilerError> {
    let context = inkwell::context::Context::create();
    let mut function = Function::new(Arity::None, ast_function.name.clone());
    let mut compiler = Compiler::new(&mut function, 0, &context);

    compiler.compile(&ast_function)?;

    // let globals = compiler.end();
    // Ok((function, globals))
    Ok(())
}
