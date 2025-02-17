mod builder;
mod compiler;
mod error;

use std::path::Path;

use crate::compiler::CompilerContext;
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

    let mut store = CompilerContext::new();
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

    compiler.execute()
}

pub fn compile_and_build_binary(
    ast_function: &vif_objects::ast::Function,
) -> Result<(), CompilerError> {
    let mut function = Function::new(Arity::None, ast_function.name.clone());
    let context = inkwell::context::Context::create();

    let compiler = compile(ast_function, &mut function, &context)?;

    compiler.build_binary("here.o")
}

pub fn execute_llvm_from_stdin() -> Result<(), CompilerError> {
    let context = inkwell::context::Context::create();
    let buffer = inkwell::memory_buffer::MemoryBuffer::create_from_stdin()
        .map_err(|e| CompilerError::LLVM("Could not create memory buffer".to_owned()))?;

    let module = context.create_module_from_ir(buffer).unwrap();

    let engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|_| CompilerError::LLVM("Could not start JIT engine".to_owned()))?;

    let function = module.get_function("main").unwrap();

    unsafe { engine.run_function(function, &[]) };

    Ok(())
}
