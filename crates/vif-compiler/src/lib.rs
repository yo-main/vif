mod compiler;
mod debug;
mod error;

use crate::error::CompilerError;
use compiler::Compiler;
pub use debug::disassemble_application;
use vif_objects::function::Arity;
use vif_objects::function::Function;
use vif_objects::function::NativeFunction;
use vif_objects::function::NativeFunctionCallee;
use vif_objects::global::Global;
use vif_objects::global_store::GlobalStore;
use vif_objects::op_code::OpCode;

pub fn compile(
    ast_function: &vif_objects::ast::Function,
) -> Result<(Function, GlobalStore), CompilerError> {
    let mut function = Function::new(Arity::None, ast_function.name.clone());
    let mut compiler = Compiler::new(&mut function, 0);

    compiler.compile(&ast_function)?;

    let globals = compiler.end();
    Ok((function, globals))
}
