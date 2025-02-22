use crate::{error::Error, parser::Parser};

pub trait AndParse<I, A> {
    fn and<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)>;
}

impl<I, A, T: Parser<I, A>> AndParse<I, A> for T {
    fn and<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)> {
        move |i| self(i).and_then(|(i, o1)| parser(i).map(|(i, o2)| (i, (o1, o2))))
    }
}

pub trait AndParseStr<'a, A> {
    fn trim_and<B>(self, parser: impl Parser<&'a str, B>) -> impl Parser<&'a str, (A, B)>;
    fn char_and(self, char: char) -> impl Parser<&'a str, A>;
}

impl<'a, A, P: Parser<&'a str, A>> AndParseStr<'a, A> for P {

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
    
    fn trim_and<B>(self, parser: impl Parser<&'a str, B>) -> impl Parser<&'a str, (A, B)> {
        move |i| self(i).and_then(|(i, o1)| parser(i.trim_start()).map(|(i, o2)| (i, (o1, o2))))
    }
}

#[cfg(test)]
mod tests {
    use super::{AndParse, AndParseStr};
    use crate::parser::{self, char, none, num, trimer};

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

    #[test]
    fn test4() {
        let base = "   +  123+abc";
        let parser = trimer.char_and('+').trim_and(num).char_and('+');
        assert_eq!(parser(base), Ok(("abc", ((), 123))))
    }
}
