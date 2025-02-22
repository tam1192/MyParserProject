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
    fn char_rand(self, char: char) -> impl Parser<&'a str, A>;
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
    
    fn char_rand(self, char: char) -> impl Parser<&'a str, A> {
        move |i| {
            if i.starts_with(char) {
                self(&i[1..])
            } else {
                Err(Error::ParseCharError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AndParse, AndParseStr};
    use crate::parser::{char, num, trim_and};

    #[test]
    fn test2() {
        let base = "123+abc";
        let parser = num.and(char('+'));
        assert_eq!(parser(base), Ok(("abc", (123, ()))))
    }

    #[test]
    fn test3() {
        let base = "   123+abc";
        let parser = trim_and(num).and(char('+'));
        assert_eq!(parser(base), Ok(("abc", (123, ()))))
    }

    #[test]
    fn test4() {
        let base = "   +  123+abc";
        let parser = trim_and(char('+').trim_and(num).char_and('+'));
        assert_eq!(parser(base), Ok(("abc", ((), 123))))
    }

    #[test]
    fn test5() {
        let base = "+-123\n-123+\n+abc";
        let parser = num.char_rand('-').char_rand('+').trim_and(num.char_rand('-').char_and('+')).trim_and(char('+'));
        assert_eq!(parser(base), Ok(("abc", ((123, 123), ()))))
    }

    // #[test]
    // fn test6() {
    //     let base = "( 123 )";
    //     let parser1 = trim_and(num).char_rand('(');
    //     assert_eq!(parser1(base), Ok((" )", 123)));
    //     // let parser2 = parser1.trim_and(char_and(c, parser));
    //     // assert_eq!(parser2(base), Ok((")", 123)))
    // }
}
