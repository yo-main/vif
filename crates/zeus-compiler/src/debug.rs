use crate::chunk::Chunk;
use crate::op_code::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    chunk
        .iter()
        .enumerate()
        .for_each(|(i, c)| disassemble_instruction(i, c, chunk))
}

pub fn disassemble_instruction(offset: usize, op_code: &OpCode, chunk: &Chunk) {
    print!("{:0>4} ", offset);
    if offset > 0 && chunk.get_line(offset - 1) == chunk.get_line(offset) {
        print!("{char:>4} ", char = "|",);
    } else {
        print!("{char:>4} ", char = chunk.get_line(offset),);
    }

    match op_code {
        OpCode::OP_RETURN => simple_instruction("OP_RETURN", offset),
        OpCode::OP_CONSTANT(i) => constant_instruction("OP_CONSTANT", chunk, *i, offset),
        OpCode::OP_NEGATE => simple_instruction("OP_NEGATE", offset),
        OpCode::OP_ADD => simple_instruction("OP_ADD", offset),
        OpCode::OP_SUBSTRACT => simple_instruction("OP_SUBSTRACT", offset),
        OpCode::OP_MULTIPLY => simple_instruction("OP_MULTIPLY", offset),
        OpCode::OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
        OpCode::OP_MODULO => simple_instruction("OP_MODULO", offset),
        OpCode::OP_TRUE => simple_instruction("OP_TRUE", offset),
        OpCode::OP_FALSE => simple_instruction("OP_FALSE", offset),
        OpCode::OP_NONE => simple_instruction("OP_NONE", offset),
        OpCode::OP_NOT => simple_instruction("OP_NOT", offset),
        OpCode::OP_EQUAL => simple_instruction("OP_EQUAL", offset),
        OpCode::OP_NOT_EQUAL => simple_instruction("OP_NOT_EQUAL", offset),
        OpCode::OP_GREATER => simple_instruction("OP_GREATER", offset),
        OpCode::OP_GREATER_OR_EQUAL => simple_instruction("OP_GREATER_OR_EQUAL", offset),
        OpCode::OP_LESS => simple_instruction("OP_LESS", offset),
        OpCode::OP_LESS_OR_EQUAL => simple_instruction("OP_LESS_OR_EQUAL", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) {
    println!("{}", name);
}

fn constant_instruction(name: &str, chunk: &Chunk, index: usize, offset: usize) {
    println!(
        "{} {:0>4} {}",
        name,
        offset,
        chunk.get_constant(index).unwrap()
    );
}
