mod common;

#[test]
fn zeus_tests() {
    let test_folders = std::env::current_dir()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join("../../tests");

    for file in std::fs::read_dir(test_folders).unwrap() {
        println!("{:?}", file);
        let content = std::fs::read_to_string(file.unwrap().path()).unwrap();
        let result = zeus_vm::interpret(content);
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}
