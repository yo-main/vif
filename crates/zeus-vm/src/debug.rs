use crate::error::ZeusError;
use crate::opcode::Chunk;
use crate::opcode::OpCode;
use crate::value::Value;

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
        OpCode::OP_TEST(v) => negate_instruction("OP_TEST", v, offset),
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

fn negate_instruction(name: &str, value: &Value, offset: usize) {
    println!("{} {:0>4} {}", name, offset, value);
}
