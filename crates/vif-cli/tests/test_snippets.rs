use vif_ast::build_ast;
use vif_compiler::compile;
use vif_objects::op_code::ItemReference;
use vif_objects::op_code::OpCode;
use vif_objects::span::Span;
use vif_typing::run_typing_checks;
use vif_vm::interpret;

#[test]
fn test_variable_declaration() {
    let string = "
        var i = 1
        print(i)
";

    let bytes = vec![
        OpCode::Global(3),                        // constant 1
        OpCode::CreateLocal(0),                   // save 1 in a variable named i
        OpCode::GetGlobal(1),                     // get print
        OpCode::GetLocal(0),                      // get i
        OpCode::Call(1),                          // call print
        OpCode::Pop,                              // pop print function
        OpCode::None(ItemReference::new(None)),   // implicit None
        OpCode::Return(ItemReference::new(None)), // return
    ];

    let mut ast = build_ast(string).unwrap();
    run_typing_checks(&mut ast).unwrap();
    let (function, globals) = compile(&ast).unwrap();
    assert_eq!(function.chunk.code, bytes);

    let result = interpret(function, globals, string);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}

#[test]
fn test_simple() {
    let string = "
        var mut i = 1
        while i < 5:
            print(i)
            i = i + 1
";

    let bytes = vec![
        OpCode::Global(3),                                        // constant 1
        OpCode::CreateLocal(0),                                   // save 1 in a variable named i
        OpCode::GetLocal(0),                                      // get i
        OpCode::Global(4),                                        // get constant 5
        OpCode::Less(ItemReference::new(Some(Span::new(3, 19)))), // substract them
        OpCode::JumpIfFalse(11),                                  // if branch
        OpCode::Pop,                                              // pop jump op (if is true)
        OpCode::GetGlobal(1),                                     // get print
        OpCode::GetLocal(0),                                      // get i
        OpCode::Call(1),                                          // call print
        OpCode::Pop,                                              // pop print op
        OpCode::GetLocal(0),                                      // get i
        OpCode::Global(5),                                        // get constant 1
        OpCode::Add(ItemReference::new(Some(Span::new(5, 21)))),  // add them
        OpCode::SetLocal(0),                                      // store them in a new global
        OpCode::Pop,                                              // pop set op
        OpCode::Goto(2),                                          // while loop
        OpCode::Pop,                                              // pop while cond value
        OpCode::None(ItemReference::new(None)),                   // Implicit none
        OpCode::Return(ItemReference::new(None)),                 // return
    ];

    let mut ast = build_ast(string).unwrap();
    run_typing_checks(&mut ast).unwrap();
    let (function, globals) = compile(&ast).unwrap();
    assert_eq!(function.chunk.code, bytes);

    let result = interpret(function, globals, string);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
