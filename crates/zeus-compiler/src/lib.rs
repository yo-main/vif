mod compiler;
mod debug;
mod error;
mod parser_rule;
mod precedence;

pub use crate::error::CompilerError;
use compiler::Compiler;
use zeus_ast::build_ast;
pub use zeus_objects::chunk::Chunk;
pub use zeus_objects::function::Arity;
pub use zeus_objects::function::Function;
pub use zeus_objects::function::NativeFunction;
pub use zeus_objects::function::NativeFunctionCallee;
use zeus_objects::global::Global;
pub use zeus_objects::op_code::OpCode;
pub use zeus_objects::variable::Variable;
use zeus_scanner::Scanner;

pub fn compile(content: String) -> Result<(Function, Global), CompilerError> {
    let ast = build_ast(content);
    let mut function = Function::new(Arity::None, "Main".to_owned());
    // let mut scanner = Scanner::new(content.as_str());
    let mut compiler = Compiler::new(&mut function);

    compiler.compile(&ast);

    let globals = compiler.end();
    Ok((function, globals))
}
