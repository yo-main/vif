mod chunk;
mod constant;
mod debug;
mod error;
mod op_code;

pub use crate::chunk::Chunk;
pub use crate::constant::Constant;
pub use crate::error::CompilerError;
pub use crate::op_code::OpCode;

pub fn compile(content: String) -> Result<Chunk, CompilerError> {
    Ok(Chunk::new())
}
