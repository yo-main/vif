mod builder;
mod compiler;
mod error;

use crate::compiler::CompilerContext;
use crate::error::CompilerError;
use compiler::Compiler;

use inkwell;
use inkwell::context::Context;

fn compile<'func, 'ctx>(
    ast_function: &vif_objects::ast::Function,
    context: &'ctx Context,
) -> Result<Compiler<'ctx>, CompilerError> {
    let compiler = Compiler::new(&context);

    let mut store = CompilerContext::new();
    compiler.add_builtin_functions(&mut store);
    compiler.compile(&ast_function, &mut store)?;
    compiler.add_return_main_function()?;

    Ok(compiler)
}

pub fn get_llvm_ir(ast_function: &vif_objects::ast::Function) -> Result<String, CompilerError> {
    let context = inkwell::context::Context::create();

    let compiler = compile(ast_function, &context)?;

    Ok(compiler.as_string())
}

pub fn compile_and_execute(ast_function: &vif_objects::ast::Function) -> Result<(), CompilerError> {
    let context = inkwell::context::Context::create();

    let compiler = compile(ast_function, &context)?;

    compiler.execute()
}

pub fn compile_and_build_binary(
    ast_function: &vif_objects::ast::Function,
) -> Result<(), CompilerError> {
    let context = inkwell::context::Context::create();

    let compiler = compile(ast_function, &context)?;

    compiler.build_binary("here.o")
}

pub fn execute_llvm_from_stdin() -> Result<(), CompilerError> {
    let context = inkwell::context::Context::create();
    let buffer = inkwell::memory_buffer::MemoryBuffer::create_from_stdin()
        .map_err(|_| CompilerError::LLVM("Could not create memory buffer".to_owned()))?;

    let module = context.create_module_from_ir(buffer).unwrap();

    let engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|_| CompilerError::LLVM("Could not start JIT engine".to_owned()))?;

    let function = module.get_function("main").unwrap();

    unsafe { engine.run_function(function, &[]) };

    Ok(())
}
