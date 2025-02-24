use crate::parser::Parser;

#[derive(Debug, PartialEq)]
pub enum OrResult<A, B> {
    A(A),
    B(B),
}

pub trait OrParse<I, O> {
    fn or(self, parser: impl Parser<I, O>) -> impl Parser<I, O>;
    fn or_ab<O2>(self, parser: impl Parser<I, O2>) -> impl Parser<I, OrResult<O, O2>>;
}

impl<I: Copy, O, T: Parser<I, O>> OrParse<I, O> for T {
    fn or(self, parser: impl Parser<I, O>) -> impl Parser<I, O> {
        move |i| self(i).or_else(|_| parser(i))
    }

    fn or_ab<O2>(self, parser: impl Parser<I, O2>) -> impl Parser<I, OrResult<O, O2>> {
        move |i| {
            let x = match self(i) {
                Ok((i, o)) => (i, OrResult::A(o)),
                Err(_) => {
                    let (i, o2) = parser(i)?;
                    (i, OrResult::B(o2))
                }
            };
            Ok(x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OrParse;
    use crate::parser::{char, num, AndParse, MapParse};

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

    #[test]
    fn test3() {
        let base = "cdefg";
        let parser = char('a').or(char('b')).or(char('c'));
        assert_eq!(parser(base), Ok(("defg", ())))
    }

    #[test]
    fn test4() {
        let base1 = "+123abc";
        let base2 = "-123abc";
        let parser = char('+').or_ab(char('-')).and(num).map(|(o, n)| match o {
            super::OrResult::A(_) => n,
            super::OrResult::B(_) => 0 - n,
        });
        assert_eq!(parser(base1), Ok(("abc", 123)));
        assert_eq!(parser(base2), Ok(("abc", -123)));
    }
}
