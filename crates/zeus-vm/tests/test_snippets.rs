mod common;
use zeus_objects::op_code::OpCode;

#[test]
fn test_variable_declaration() {
    let string = "
        var i = 1
        print(i)
"
    .to_owned();
    let bytes = vec![
        OpCode::Constant(1),       // constant 1
        OpCode::GlobalVariable(0), // save 1 in a variable named i
        OpCode::GetGlobal(2),      // get print
        OpCode::GetGlobal(3),      // get i
        OpCode::Call(1),           // call print
        OpCode::Pop,               // pop print function
        OpCode::None,              // implicit None
        OpCode::Return,            // return
    ];

    let result = common::interpret(string, bytes);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}

#[test]
fn test_simple() {
    let string = "
        var i = 1
        while i < 5:
            print(i)
            i = i + 1
"
    .to_owned();

    let bytes = vec![
        OpCode::Constant(1),       // constant 1
        OpCode::GlobalVariable(0), // save 1 in a variable named i
        OpCode::GetGlobal(2),      // get i
        OpCode::Constant(3),       // get constant 5
        OpCode::Less,              // substract them
        OpCode::JumpIfFalse(11),   // if branch
        OpCode::Pop,               // pop jump op (if is true)
        OpCode::GetGlobal(4),      // get print
        OpCode::GetGlobal(5),      // get i
        OpCode::Call(1),           // call print
        OpCode::Pop,               // pop print op
        OpCode::GetGlobal(7),      // get i
        OpCode::Constant(8),       // get constant 1
        OpCode::Add,               // add them
        OpCode::SetGlobal(6),      // store them in a new global
        OpCode::Pop,               // pop set op
        OpCode::Goto(2),           // while loop
        OpCode::Pop,               // pop while cond value
        OpCode::None,              // Implicit none
        OpCode::Return,            // return
    ];

    let result = common::interpret(string, bytes);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
