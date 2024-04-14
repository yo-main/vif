#[test]
fn vif_tests() {
    use vif_ast::build_ast;
    use vif_compiler::compile;
    use vif_typing::run_typing_checks;
    let test_folders = std::env::current_dir()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join("../../tests");

    for file in std::fs::read_dir(test_folders).unwrap() {
        println!("{:?}", file);
        let content = std::fs::read_to_string(file.unwrap().path()).unwrap();

        let ast = build_ast(content.as_str()).unwrap();
        let typed_ast = run_typing_checks(ast).unwrap();
        let (f, g) = compile(&typed_ast).unwrap();

        let result = vif_vm::interpret(f, g);
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}
