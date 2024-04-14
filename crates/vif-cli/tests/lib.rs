#[test]
fn vif_tests() {
    use vif_compiler::compile;
    let test_folders = std::env::current_dir()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join("../../tests");

    for file in std::fs::read_dir(test_folders).unwrap() {
        println!("{:?}", file);
        let content = std::fs::read_to_string(file.unwrap().path()).unwrap();
        let (f, g) = compile(content.as_str()).unwrap();
        let result = vif_vm::interpret(f, g);
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}
