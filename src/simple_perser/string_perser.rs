fn ex(input: &str) -> (&str, &str) {
    return ("", input);
}

#[test]
fn test_ex() {
    assert_eq!(ex("abc"), ("", "abc"));
}