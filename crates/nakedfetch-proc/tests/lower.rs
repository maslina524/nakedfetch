use proc::lower;

#[test]
fn lower_basic_test() {
    assert_eq!(lower!("Hello, World!"), "hello, world!");
}

#[test]
fn lower_with_stringify_test() {
    assert_eq!(lower!(HelloWorld), "helloworld");
}
