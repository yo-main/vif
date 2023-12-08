mod compiler;
mod debug;
mod error;
mod parser_rule;
mod precedence;

pub use crate::error::CompilerError;
use compiler::Compiler;
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
    let mut function = Function::new(Arity::None, "Main".to_owned());
    let mut scanner = Scanner::new(content.as_str());
    let mut compiler = Compiler::new(&mut scanner, &mut function);
    let mut errors = Vec::new();

    loop {
        log::debug!("Main compiler loop");
        match compiler.declaration() {
            Err(CompilerError::EOF) => break,
            Err(e) => {
                log::error!("Compile error received in main loop: {}", e);
                errors.push(e);
                match compiler.synchronize() {
                    Ok(_) => (),
                    Err(CompilerError::EOF) => break,
                    Err(e) => return Err(e),
                };
            }
            Ok(_) => (),
        };
    }

    if !errors.is_empty() {
        errors.reverse();
        return Err(errors.pop().unwrap());
    }

    let globals = compiler.end();
    Ok((function, globals))
}
