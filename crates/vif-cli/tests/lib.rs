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

        let mut ast = build_ast(content.as_str()).unwrap();
        run_typing_checks(&mut ast).unwrap();
        let (f, g) = compile(&ast).unwrap();

        let result = vif_vm::interpret(f, g, content.as_str());
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}
