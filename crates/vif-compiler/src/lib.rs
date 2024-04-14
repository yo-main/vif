mod compiler;
mod debug;
mod error;

use crate::error::CompilerError;
use compiler::Compiler;
pub use debug::disassemble_application;
use vif_ast::build_ast;
use vif_objects::function::Arity;
use vif_objects::function::Function;
use vif_objects::function::NativeFunction;
use vif_objects::function::NativeFunctionCallee;
use vif_objects::global::Global;
use vif_objects::global_store::GlobalStore;
use vif_objects::op_code::OpCode;
use vif_typing::run_typing_checks;

pub fn compile(content: &str) -> Result<(Function, GlobalStore), CompilerError> {
    let ast = match build_ast(content) {
        Ok(ast) => ast,
        Err(errors) => {
            for error in errors.iter() {
                println!("AST error: {error}");
            }

            return Err(CompilerError::SyntaxError(format!("{}", errors[0])));
        }
    };

    let ast = run_typing_checks(ast).unwrap();
    let mut function = Function::new(Arity::None, ast.name.clone());
    let mut compiler = Compiler::new(&mut function, 0);

    compiler.compile(&ast)?;

    let globals = compiler.end();
    Ok((function, globals))
}
