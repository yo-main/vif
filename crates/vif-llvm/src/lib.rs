mod builder;
mod compiler;
mod error;

use crate::compiler::Store;
use crate::error::CompilerError;
use compiler::Compiler;

use inkwell;
use inkwell::context::Context;
use vif_objects::function::Arity;
use vif_objects::function::Function;
use vif_objects::function::NativeFunction;
use vif_objects::function::NativeFunctionCallee;
use vif_objects::global::Global;
use vif_objects::op_code::OpCode;

fn compile<'func, 'ctx>(
    ast_function: &vif_objects::ast::Function,
    function: &'func mut Function,
    context: &'ctx Context,
) -> Result<Compiler<'func, 'ctx>, CompilerError> {
    let compiler = Compiler::new(function, &context);

    let mut store = Store::new();
    compiler.add_builtin_functions(&mut store);
    compiler.compile(&ast_function, &mut store)?;
    compiler.add_return_main_function()?;

    Ok(compiler)
}

pub fn get_llvm_ir(ast_function: &vif_objects::ast::Function) -> Result<String, CompilerError> {
    let mut function = Function::new(Arity::None, ast_function.name.clone());
    let context = inkwell::context::Context::create();

    let compiler = compile(ast_function, &mut function, &context)?;

    Ok(compiler.as_string())
}

pub fn compile_and_execute(ast_function: &vif_objects::ast::Function) -> Result<(), CompilerError> {
    let mut function = Function::new(Arity::None, ast_function.name.clone());
    let context = inkwell::context::Context::create();

    let compiler = compile(ast_function, &mut function, &context)?;

    compiler.execute(&ast_function)
}
