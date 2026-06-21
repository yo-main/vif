mod builder;
mod compiler;
mod error;

use crate::compiler::CompilerContext;
use crate::error::CompilerError;
use compiler::Compiler;

use vif_typing::Entrypoint;

use inkwell;
use inkwell::context::Context;

fn compile<'func, 'ctx>(
    entrypoint: &Entrypoint,
    context: &'ctx Context,
) -> Result<Compiler<'ctx>, CompilerError> {
    let compiler = Compiler::new(&context);
    let mut store = CompilerContext::new();

    compiler.add_builtin_functions(&mut store);
    compiler.compile_entrypoint(&entrypoint, &mut store)?;

    Ok(compiler)
}

pub fn get_llvm_ir(entrypoint: &Entrypoint) -> Result<String, CompilerError> {
    let context = inkwell::context::Context::create();

    let compiler = compile(entrypoint, &context)?;

    Ok(compiler.as_string())
}

pub fn compile_and_execute(entrypoint: &Entrypoint) -> Result<(), CompilerError> {
    let context = inkwell::context::Context::create();

    let compiler = compile(entrypoint, &context)?;

    compiler.execute()
}

pub fn compile_and_build_binary(entrypoint: &Entrypoint) -> Result<(), CompilerError> {
    let context = inkwell::context::Context::create();

    let compiler = compile(entrypoint, &context)?;

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
