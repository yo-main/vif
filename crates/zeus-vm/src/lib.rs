use error::InterpreterError;
use opcode::Chunk;

mod debug;
mod error;
pub mod opcode;
pub mod value;
mod vm;

pub fn interpret(content: String) -> Result<(), InterpreterError> {
    let mut chunk = Chunk::new();
    let mut stack = Vec::new();
    let vm = vm::VM::new();

    chunk.compile(content)?;

    for byte in chunk.iter() {
        vm.interpret(byte, &mut stack, &chunk.constants)?;
    }

    Ok(())
}
