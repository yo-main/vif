mod compiler;
mod debug;
mod error;

pub use crate::error::CompilerError;
use compiler::Compiler;
use vif_ast::build_ast;
pub use vif_objects::chunk::Chunk;
pub use vif_objects::function::Arity;
pub use vif_objects::function::Function;
pub use vif_objects::function::NativeFunction;
pub use vif_objects::function::NativeFunctionCallee;
use vif_objects::global::GlobalStore;
pub use vif_objects::op_code::OpCode;
pub use vif_objects::variable::Variable;

pub fn compile(content: String) -> Result<(Function, GlobalStore), CompilerError> {
    let ast = build_ast(content).unwrap();
    let mut function = Function::new(Arity::None, "Main".to_owned());
    // let mut scanner = Scanner::new(content.as_str());
    let mut compiler = Compiler::new(&mut function, 0);

    compiler.compile(&ast)?;

    let globals = compiler.end();
    Ok((function, globals))
}
