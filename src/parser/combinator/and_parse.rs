use crate::parser::{error::Error, Parser};

pub trait AndParse<I, A> {
    fn and<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)>;
    fn and_a<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, A>;
    fn and_b<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, B>;
}

impl<I, A, T: Parser<I, A>> AndParse<I, A> for T {
    fn and<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)> {
        move |i| match self(i) {
            Ok((i, o1)) => match parser(i) {
                Ok((i, o2)) => Ok((i, (o1, o2))),
                Err(e) => Err(Error::CombinatorParseError(Box::new(e), None)),
            },
            Err(e) => Err(Error::CombinatorParseError(Box::new(e), None)),
        }
    }

    fn and_a<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, A> {
        move |i| match self(i) {
            Ok((i, o1)) => match parser(i) {
                Ok((i, _)) => Ok((i, o1)),
                Err(e) => Err(Error::CombinatorParseError(Box::new(e), None)),
            },
            Err(e) => Err(Error::CombinatorParseError(Box::new(e), None)),
        }
    }

    fn and_b<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, B> {
        move |i| match self(i) {
            Ok((i, _)) => match parser(i) {
                Ok((i, o2)) => Ok((i, o2)),
                Err(e) => Err(Error::CombinatorParseError(Box::new(e), None)),
            },
            Err(e) => Err(Error::CombinatorParseError(Box::new(e), None)),
        }
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
