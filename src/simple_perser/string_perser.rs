fn ex() -> String {
    let s = "hello";
    s.to_string()
}

#[test]
fn test_ex() {
    assert_eq!(ex(), "hello");
}