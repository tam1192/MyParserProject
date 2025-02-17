type Output<'a, T> = Option<(T, &'a str)>;
type Parser<T> = Box<dyn Fn(&str) -> Output<T>>;

fn char(i: char) -> Parser<()> {
    Box::new(move |input: &str| {
        if let Some(c) = input.chars().next() {
            if c == i {
                return Some(((), &input[1..]));
            }
        }
        return None;
    })
}

#[cfg(test)]
mod tests_char {
    use super::char;
    #[test]
    fn t1() {
        let input = "abc";
        let parser = char('a');
        assert_eq!(parser(input), Some(((), "bc")));
    }

    #[test]
    fn t2() {
        let input = "abc";
        let parser = char('b');
        assert_eq!(parser(input), None);
    }

    #[test]
    fn t3() {
        let input = "";
        let parser = char(' ');
        assert_eq!(parser(input), None);
    }
}
