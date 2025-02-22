use crate::{error::Error, parser::{base, Parser}};

pub trait AndParse<I, A> {
    fn and<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)>;
}

impl<I, A, T: Parser<I, A>> AndParse<I, A> for T {
    fn and<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)> {
        move |i| self(i).and_then(|(i, o1)| parser(i).map(|(i, o2)| (i, (o1, o2))))
    }
}

pub trait AndParseStr<'a, A> {
    fn trim_and(self) -> impl Parser<&'a str, A>;
    fn char_and(self, char: char) -> impl Parser<&'a str, A>;
}

impl<'a, A, P: Parser<&'a str, A>> AndParseStr<'a, A> for P {
    fn trim_and(self) -> impl Parser<&'a str, A> {
        move |i| self(i.trim_start())
    }

    fn char_and(self, char: char) -> impl Parser<&'a str, A> {
        move |i| {
            let (i, a) = self(i)?;
            if i.starts_with(char) {
                Ok((&i[1..], a))
            } else {
                Err(Error::ParseCharError)
            }
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
        let parser = trimer.and(num).and(char('+'));
        assert_eq!(parser(base), Ok(("abc", (((), 123), ()))))
    }
}
