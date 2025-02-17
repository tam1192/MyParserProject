// type Parser<'a> = Box<dyn Fn() -> Output<'a>>;


/// 文字列を指定したパターンで分割する
fn char(input: &str, char: char) -> Option<((), &str)> {
    if let Some(c) = input.chars().next() {
        if c == char {
            return Some(((), &input[1..]));
        }
    }
    return None;
}

#[cfg(test)]
mod tests_ex {
    use super::char;
    #[test]
    fn t1() {
        let input = "abc";
        assert_eq!(char(input, 'a'), Some(((), "bc")));
    }

    #[test]
    fn t2() {
        let input = "abc";
        assert_eq!(char(input, 'b'), None);
    }

    #[test]
    fn t3() {
        let input = "";
        assert_eq!(char(input, ' '), None);
    }
}