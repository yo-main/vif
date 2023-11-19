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
        OpCode::Return => simple_instruction("OP_RETURN", offset),
        OpCode::Constant(i) => constant_instruction("OP_CONSTANT", chunk, *i, offset),
        OpCode::GlobalVariable(i) => constant_instruction("OP_GLOBAL_VARIABLE", chunk, *i, offset),
        OpCode::GetGlobal(i) => constant_instruction("OP_GET_GLOBAL", chunk, *i, offset),
        OpCode::SetGlobal(i) => constant_instruction("OP_SET_GLOBAL", chunk, *i, offset),
        OpCode::GetLocal(i) => constant_instruction("OP_GET_LOCAL", chunk, *i, offset),
        OpCode::SetLocal(i) => constant_instruction("OP_SET_LOCAL", chunk, *i, offset),
        OpCode::Negate => simple_instruction("OP_NEGATE", offset),
        OpCode::Add => simple_instruction("OP_ADD", offset),
        OpCode::Substract => simple_instruction("OP_SUBSTRACT", offset),
        OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
        OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
        OpCode::Modulo => simple_instruction("OP_MODULO", offset),
        OpCode::True => simple_instruction("OP_TRUE", offset),
        OpCode::False => simple_instruction("OP_FALSE", offset),
        OpCode::None => simple_instruction("OP_NONE", offset),
        OpCode::Not => simple_instruction("OP_NOT", offset),
        OpCode::Equal => simple_instruction("OP_EQUAL", offset),
        OpCode::NotEqual => simple_instruction("OP_NOT_EQUAL", offset),
        OpCode::Greater => simple_instruction("OP_GREATER", offset),
        OpCode::GreaterOrEqual => simple_instruction("OP_GREATER_OR_EQUAL", offset),
        OpCode::Less => simple_instruction("OP_LESS", offset),
        OpCode::LessOrEqual => simple_instruction("OP_LESS_OR_EQUAL", offset),
        OpCode::Print => simple_instruction("OP_PRINT", offset),
        OpCode::Pop => simple_instruction("OP_POP", offset),
        OpCode::JumpIfFalse(i) => jump_instruction("OP_JUMP_IF_FALSE", i, offset),
        OpCode::Jump(i) => jump_instruction("OP_JUMP", i, offset),
        OpCode::Goto(i) => jump_instruction("OP_LOOP", i, offset),
    }
}

fn simple_instruction(name: &str, _offset: usize) {
    println!("{}", name);
}

fn jump_instruction(name: &str, jump: &usize, _offset: usize) {
    println!("{} {}", name, jump);
}

fn constant_instruction(name: &str, chunk: &Chunk, index: usize, offset: usize) {
    println!(
        "{} {:0>4} {}",
        name,
        offset,
        chunk.get_constant(index).unwrap()
    );
}
