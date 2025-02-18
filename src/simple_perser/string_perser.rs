type Output<'a, T> = Option<(T, &'a str)>;
type Parser<T> = Box<dyn Fn(&str) -> Output<T>>;

fn number(i: &str) -> Output<i32> {
    let mut num = 0;
    let mut rest = i;
    for c in i.chars() {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as i32;
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if rest.len() == i.len() {
        return None;
    }
    return Some((num, rest));
}

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

fn many(p: Parser<()>) -> Parser<()> {
    Box::new(move |input: &str| {
        let mut rest = input;
        while let Some((_, r)) = p(rest) {
            rest = r;
        }
        return Some(((), rest));
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

mod tests_number {
    use super::number;
    #[test]
    fn t1() {
        let input = "123abc";
        assert_eq!(number(input), Some((123, "abc")));
    }

    #[test]
    fn t2() {
        let input = "abc";
        assert_eq!(number(input), None);
    }

    #[test]
    fn t3() {
        let input = "";
        assert_eq!(number(input), None);
    }
}

mod tests_many {
    use super::{char, many};
    #[test]
    fn t1() {
        let input = "aaabbbccc";
        let parser = many(char('a'));
        assert_eq!(parser(input), Some(((), "bbbccc")));
    }

    #[test]
    fn t2() {
        let input = "aaabbbccc";
        let parser = many(char('b'));
        assert_eq!(parser(input), None);
    }

    #[test]
    fn t3() {
        let input = "";
        let parser = many(char(' '));
        assert_eq!(parser(input), Some(((), "")));
    }
}