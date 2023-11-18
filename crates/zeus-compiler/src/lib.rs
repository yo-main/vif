mod chunk;
mod compiler;
mod debug;
mod error;
mod local;
mod op_code;
mod parser_rule;
mod precedence;
mod variable;

pub use crate::chunk::Chunk;
pub use crate::error::CompilerError;
pub use crate::op_code::OpCode;
pub use crate::variable::Variable;
use compiler::Compiler;
use zeus_scanner::Scanner;

pub fn compile(content: String) -> Result<Chunk, CompilerError> {
    let scanner = Scanner::new(content.as_str());
    let mut compiler = Compiler::new(scanner);

    loop {
        log::debug!("Main compiler loop");
        match compiler.declaration() {
            Err(CompilerError::EOF) => break,
            Err(e) => {
                log::error!("Compile error received in main loop: {}", e);
                match compiler.synchronize() {
                    Ok(_) => (),
                    Err(CompilerError::EOF) => break,
                    Err(e) => return Err(e),
                };
            }
            Ok(_) => (),
        };
    }

    compiler.end();

    Ok(compiler.compiling_chunk)
}
