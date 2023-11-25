mod chunk;
mod compiler;
mod debug;
mod error;
mod function;
mod local;
mod op_code;
mod parser_rule;
mod precedence;
mod variable;

pub use crate::chunk::Chunk;
pub use crate::error::CompilerError;
pub use crate::function::Function;
pub use crate::op_code::OpCode;
pub use crate::variable::Variable;
pub use compiler::Application;
use compiler::Compiler;
use zeus_scanner::Scanner;

pub fn compile(content: String) -> Result<Application, CompilerError> {
    let mut scanner = Scanner::new(content.as_str());
    let mut compiler = Compiler::new(&mut scanner, Application::new());
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
    Ok(compiler.end())
}
