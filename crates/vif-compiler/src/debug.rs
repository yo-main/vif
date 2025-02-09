use vif_objects::chunk::Chunk;
use vif_objects::function::Function;
use vif_objects::global::Global;
use vif_objects::global_store::GlobalStore;
use vif_objects::op_code::OpCode;

pub fn disassemble_application(function: &Function, globals: &GlobalStore) {
    let functions = globals
        .as_vec()
        .iter()
        .filter_map(|g| match g {
            Global::Function(f) => Some(f),
            _ => None,
        })
        .collect::<Vec<&Box<Function>>>();

    for f in functions {
        disassemble_function(f, globals);
    }

    disassemble_function(function, globals);
}

fn disassemble_function(function: &Function, globals: &GlobalStore) {
    println!("== {} ==", function.name);
    function
        .chunk
        .iter(0)
        .enumerate()
        .for_each(|(i, c)| disassemble_instruction(i, c, &function.chunk, globals));
}

fn disassemble_instruction(offset: usize, op_code: &OpCode, chunk: &Chunk, globals: &GlobalStore) {
    print!("{:0>4} ", offset);
    if offset > 0 && chunk.get_line(offset - 1) == chunk.get_line(offset) {
        print!("{char:>4} ", char = "|",);
    } else {
        print!("{char:>4} ", char = chunk.get_line(offset),);
    }

    match op_code {
        OpCode::Return(_) => simple_instruction("OP_RETURN"),
        OpCode::Global(i) => constant_instruction("OP_GLOBAL", chunk, *i, globals),
        OpCode::GlobalVariable(i) => constant_instruction("OP_GLOBAL_VARIABLE", chunk, *i, globals),
        OpCode::GetGlobal(i) => constant_instruction("OP_GET_GLOBAL", chunk, *i, globals),
        OpCode::SetGlobal(i) => constant_instruction("OP_SET_GLOBAL", chunk, *i, globals),
        OpCode::GetLocal(i) => simple_instruction(format!("OP_GET_LOCAL({i})").as_str()),
        OpCode::CreateLocal(i) => simple_instruction(format!("OP_CREATE_LOCAL({i})").as_str()),
        OpCode::SetLocal(i) => simple_instruction(format!("OP_SET_LOCAL({i})").as_str()),
        OpCode::GetInheritedLocal(v) => {
            simple_instruction(format!("OP_GET_INHERITED_LOCAL({v})").as_str())
        }
        OpCode::SetInheritedLocal(v) => {
            simple_instruction(format!("OP_SET_INHERITED_LOCAL({v})").as_str())
        }
        OpCode::Negate(_) => simple_instruction("OP_NEGATE"),
        OpCode::Add(_) => simple_instruction("OP_ADD"),
        OpCode::Substract(_) => simple_instruction("OP_SUBSTRACT"),
        OpCode::Multiply(_) => simple_instruction("OP_MULTIPLY"),
        OpCode::Divide(_) => simple_instruction("OP_DIVIDE"),
        OpCode::Modulo(_) => simple_instruction("OP_MODULO"),
        OpCode::True(_) => simple_instruction("OP_TRUE"),
        OpCode::False(_) => simple_instruction("OP_FALSE"),
        OpCode::None(_) => simple_instruction("OP_NONE"),
        OpCode::Not(_) => simple_instruction("OP_NOT"),
        OpCode::Equal(_) => simple_instruction("OP_EQUAL"),
        OpCode::NotEqual(_) => simple_instruction("OP_NOT_EQUAL"),
        OpCode::Greater(_) => simple_instruction("OP_GREATER"),
        OpCode::GreaterOrEqual(_) => simple_instruction("OP_GREATER_OR_EQUAL"),
        OpCode::Less(_) => simple_instruction("OP_LESS"),
        OpCode::LessOrEqual(_) => simple_instruction("OP_LESS_OR_EQUAL"),
        OpCode::Pop => simple_instruction("OP_POP"),
        OpCode::JumpIfFalse(i) => jump_instruction("OP_JUMP_IF_FALSE", i),
        OpCode::Jump(i) => jump_instruction("OP_JUMP", i),
        OpCode::Goto(i) => jump_instruction("OP_GOTO", i),
        OpCode::Call((i, _)) => jump_instruction("OP_CALL", i),
        OpCode::AssertTrue(_) => simple_instruction("OP_ASSERT_TRUE"),
        OpCode::NotImplemented => simple_instruction("NOT_IMPLEMENTED"),
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name);
}

fn jump_instruction(name: &str, jump: &usize) {
    println!("{} {}", name, jump);
}

fn constant_instruction(name: &str, _chunk: &Chunk, index: usize, globals: &GlobalStore) {
    println!("{} {}", name, globals.get(index));
}
