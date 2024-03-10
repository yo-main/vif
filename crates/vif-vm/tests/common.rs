use vif_compiler::compile;
use vif_objects::op_code::OpCode;
use vif_objects::stack::Stack;
use vif_objects::variable_storage::VariableStore;
use vif_vm::vm::VM;
use vif_vm::CallFrame;
use vif_vm::InterpreterError;

pub fn interpret(content: String, bytes: Vec<OpCode>) -> Result<(), InterpreterError> {
    let mut stack = Stack::new();
    let mut variables = VariableStore::new();

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

    assert_eq!(function.chunk.code, bytes);

    Ok(())
}
