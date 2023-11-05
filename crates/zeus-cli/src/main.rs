use zeus_vm::opcode;
use zeus_vm::value;

mod application;
mod cli;
mod config;
mod error;

fn setup_logging() {
    let level = std::env::var("ZEUS_LOG_LEVEL")
        .map(|lvl| lvl.parse().unwrap())
        .unwrap_or(log::LevelFilter::Trace);

    // Separate file config so we can include year, month and day in file logs
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} - {} - {} {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply();
}

fn main() {
    setup_logging();
    let config = config::get_config();

    let mut zeus = application::Zeus::init();

    let res = match config.entrypoint {
        Some(path) => zeus.run_file(path),
        _ => zeus.run_prompt(),
    };

    match res {
        Ok(_) => (),
        Err(e) => println!("Error: {e}"),
    }

    // let mut chunk = opcode::Chunk::new();
    // let cons = chunk.add_constant(value::Constant::Integer(61));
    // let cons2 = chunk.add_constant(value::Constant::Integer(2));
    // chunk.write_chunk(opcode::OpCode::OP_CONSTANT(cons), 1);
    // chunk.write_chunk(opcode::OpCode::OP_CONSTANT(cons), 1);
    // chunk.write_chunk(opcode::OpCode::OP_CONSTANT(cons), 1);
    // chunk.write_chunk(opcode::OpCode::OP_CONSTANT(cons2), 210);
    // chunk.write_chunk(opcode::OpCode::OP_ADD, 210);
    // chunk.write_chunk(opcode::OpCode::OP_SUBSTRACT, 210);
    // chunk.write_chunk(opcode::OpCode::OP_NEGATE, 210);
    // chunk.write_chunk(opcode::OpCode::OP_MULTIPLY, 210);
    // // chunk.write_chunk(opcode::OpCode::OP_ADD, 210);
    // chunk.write_chunk(opcode::OpCode::OP_RETURN, 210);
    // let mut vm = vm::VM::new();
    // match vm.interpret(&mut chunk) {
    //     Ok(v) => println!("Success"),
    //     Err(e) => println!("Error: {}", e),
    // }
    // debug::disassemble_chunk(&chunk, "test chunk");
}
