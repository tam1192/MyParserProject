use crate::parser::Parser;

pub trait OrParse<I, O> {
    fn or(self, parser: impl Parser<I, O>) -> impl Parser<I, O>;
}

impl<I: Copy, O, T: Parser<I, O>> OrParse<I, O> for T {
    fn or(self, parser: impl Parser<I, O>) -> impl Parser<I, O> {
        move |i| self(i).or_else(|_| parser(i))
    }
}

#[cfg(test)]
mod tests {
    use super::OrParse;
    use crate::parser::char;

    #[test]
    fn test1() {
        let base = ".!abc";
        let parser = char('.').or(char('!'));
        assert_eq!(parser(base), Ok(("!abc", ())));
    }

    #[test]
    fn test2() {
        let base = ".!abc";
        let parser = char('!').or(char('.'));
        assert_eq!(parser(base), Ok(("!abc", ())));
    }
}
