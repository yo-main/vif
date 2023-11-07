mod chunk;
mod compiler;
mod constant;
mod debug;
mod error;
mod op_code;
mod parser_rule;
mod precedence;

pub use crate::chunk::Chunk;
pub use crate::constant::Constant;
pub use crate::error::CompilerError;
pub use crate::op_code::OpCode;
use compiler::Compiler;
use zeus_scanner::Scanner;

pub fn compile(content: String) -> Result<Chunk, CompilerError> {
    let scanner = Scanner::new(content.as_str());
    let mut compiler = Compiler::new(scanner);

    loop {
        match compiler.expression() {
            Err(CompilerError::EOF) => break,
            Err(e) => log::error!("{}", e),
            Ok(_) => (),
        };
    }

    // compiler.consume(TokenType::EOF, "Expect end of expression")?;
    compiler.end();

    Ok(compiler.compiling_chunk)
}
