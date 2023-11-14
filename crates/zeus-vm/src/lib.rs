mod env;
mod error;
pub mod value;
mod vm;

use error::InterpreterError;
use zeus_compiler::compile;

pub fn interpret(content: String) -> Result<(), InterpreterError> {
    let vm = vm::VM::new();

    let chunk = match compile(content) {
        Ok(c) => c,
        Err(e) => return Err(InterpreterError::CompileError(format!("{}", e))),
    };

    let mut stack = Vec::new();

    for byte in chunk.iter() {
        vm.interpret(byte, &mut stack, &chunk.constants)?;
    }

    Ok(())
}
