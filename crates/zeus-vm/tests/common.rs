use std::collections::HashMap;

use zeus_compiler::compile;
use zeus_objects::op_code::OpCode;
use zeus_objects::stack::Stack;
use zeus_vm::vm::VM;
use zeus_vm::CallFrame;
use zeus_vm::InterpreterError;

pub fn interpret(content: String, bytes: Vec<OpCode>) -> Result<(), InterpreterError> {
    let mut stack = Stack::new();
    let mut variables = HashMap::new();

    let (function, globals) = match compile(content) {
        Ok(c) => c,
        Err(e) => return Err(InterpreterError::CompileError(format!("{}", e))),
    };

    let mut vm = VM::new(
        &mut stack,
        &mut variables,
        &globals,
        CallFrame::new(&function, 0, 0),
    );

    vm.interpret_loop()?;

    assert!(stack.is_empty(), "stack is not empty: {:?}", stack);
    assert_eq!(function.chunk.code, bytes);

    Ok(())
}
