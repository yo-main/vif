use crate::chunk::Chunk;
use crate::op_code::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    chunk
        .iter(0)
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
        OpCode::Return => simple_instruction("OP_RETURN"),
        OpCode::Constant(i) => constant_instruction("OP_CONSTANT", chunk, *i),
        OpCode::GlobalVariable(i) => constant_instruction("OP_GLOBAL_VARIABLE", chunk, *i),
        OpCode::GetGlobal(i) => constant_instruction("OP_GET_GLOBAL", chunk, *i),
        OpCode::SetGlobal(i) => constant_instruction("OP_SET_GLOBAL", chunk, *i),
        OpCode::GetLocal(i) => constant_instruction("OP_GET_LOCAL", chunk, *i),
        OpCode::SetLocal(i) => constant_instruction("OP_SET_LOCAL", chunk, *i),
        OpCode::Negate => simple_instruction("OP_NEGATE"),
        OpCode::Add => simple_instruction("OP_ADD"),
        OpCode::Substract => simple_instruction("OP_SUBSTRACT"),
        OpCode::Multiply => simple_instruction("OP_MULTIPLY"),
        OpCode::Divide => simple_instruction("OP_DIVIDE"),
        OpCode::Modulo => simple_instruction("OP_MODULO"),
        OpCode::True => simple_instruction("OP_TRUE"),
        OpCode::False => simple_instruction("OP_FALSE"),
        OpCode::None => simple_instruction("OP_NONE"),
        OpCode::Not => simple_instruction("OP_NOT"),
        OpCode::Equal => simple_instruction("OP_EQUAL"),
        OpCode::NotEqual => simple_instruction("OP_NOT_EQUAL"),
        OpCode::Greater => simple_instruction("OP_GREATER"),
        OpCode::GreaterOrEqual => simple_instruction("OP_GREATER_OR_EQUAL"),
        OpCode::Less => simple_instruction("OP_LESS"),
        OpCode::LessOrEqual => simple_instruction("OP_LESS_OR_EQUAL"),
        OpCode::Print => simple_instruction("OP_PRINT"),
        OpCode::Pop => simple_instruction("OP_POP"),
        OpCode::JumpIfFalse(i) => jump_instruction("OP_JUMP_IF_FALSE", i),
        OpCode::Jump(i) => jump_instruction("OP_JUMP", i),
        OpCode::Goto(i) => jump_instruction("OP_GOTO", i),
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name);
}

fn jump_instruction(name: &str, jump: &usize) {
    println!("{} {}", name, jump);
}

fn constant_instruction(name: &str, chunk: &Chunk, index: usize) {
    println!("{} {}", name, chunk.get_constant(index).unwrap());
}
