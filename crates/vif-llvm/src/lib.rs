mod builder;
mod compiler;
mod error;
mod value;

use crate::compiler::Store;
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
    let compiler = Compiler::new(&mut function, &context);

    let mut store = Store::new();
    compiler.compile(&ast_function, &mut store)?;
    compiler.add_return()?;

    compiler.print_module_to_file("here.ll");

    // let globals = compiler.end();
    // Ok((function, globals))
    Ok(())
}
