use std::error;

use crate::parser::{error::Error, Parser};

pub trait AndParse<I, A, e> {
    fn and<B>(self, parser: impl Parser<I, B, E>) -> impl Parser<I, (A, B), Error>;
    fn and_a<B>(self, parser: impl Parser<I, B, E>) -> impl Parser<I, A, Error>;
    fn and_b<B>(self, parser: impl Parser<I, B, E>) -> impl Parser<I, B, Error>;
}

impl<I, A, E, T: Parser<I, A, E>> AndParse<I, A, E> for T {
    fn and<B>(self, parser: impl Parser<I, B, E>) -> impl Parser<I, (A, B), Error> {
        move |i| self(i).and_then(|(i, o1)| parser(i).map(|(i, o2)| (i, (o1, o2))))
    }

    fn and_a<B>(self, parser: impl Parser<I, B, E>) -> impl Parser<I, A, Error> {
        move |i| self(i).and_then(|(i, o1)| parser(i).map(|(i, _)| (i, o1)))
    }

    fn and_b<B>(self, parser: impl Parser<I, B, E>) -> impl Parser<I, B, Error> {
        move |i| self(i).and_then(|(i, _)| parser(i).map(|(i, o2)| (i, o2)))
    }
}

#[cfg(test)]
mod tests {
    use super::AndParse;
    use crate::parser::{char, num, trimer};

    #[test]
    fn test2() {
        let base = "123+abc";
        let parser = num.and(char('+'));
        assert_eq!(parser(base), Ok(("abc", (123, ()))))
    }

    #[test]
    fn test3() {
        let base = "   123+abc";
        let parser = trimer.and_b(num).and_a(char('+'));
        assert_eq!(parser(base), Ok(("abc", 123)))
    }

    #[test]
    fn test4() {
        let base = "   +  123+abc";
        let parser = trimer
            .and_b(char('+'))
            .and_b(trimer)
            .and_b(num)
            .and_a(char('+'));
        assert_eq!(parser(base), Ok(("abc", 123)))
    }

    #[test]
    fn test5() {
        let base = "+-123\n-123+\n+abc";
        let parser = char('+')
            .and_b(char('-'))
            .and_b(num)
            .and(trimer.and_b(char('-')).and_b(num).and_a(char('+')))
            .and_a(trimer.and(char('+')));
        assert_eq!(parser(base), Ok(("abc", (123, 123))))
    }

    #[test]
    fn test6() {
        let base = "( 123 )";
        let parser = char('(')
            .and_b(trimer)
            .and_b(num)
            .and_a(trimer)
            .and_a(char(')'));
        assert_eq!(parser(base), Ok(("", 123)));
    }
}
