mod compiler;
mod debug;
mod error;

pub use crate::error::CompilerError;
use compiler::Compiler;
pub use debug::disassemble_application;
use vif_ast::build_ast;
pub use vif_objects::chunk::Chunk;
pub use vif_objects::function::Arity;
pub use vif_objects::function::Function;
pub use vif_objects::function::NativeFunction;
pub use vif_objects::function::NativeFunctionCallee;
pub use vif_objects::global::Global;
use vif_objects::global_store::GlobalStore;
pub use vif_objects::op_code::OpCode;
use vif_typing::run_typing_checks;

pub fn compile(content: String) -> Result<(Function, GlobalStore), CompilerError> {
    let ast = run_typing_checks(build_ast(content).unwrap());
    let mut function = Function::new(Arity::None, ast.name.clone());
    let mut compiler = Compiler::new(&mut function, 0);

    compiler.compile(&ast)?;

    let globals = compiler.end();
    Ok((function, globals))
}
