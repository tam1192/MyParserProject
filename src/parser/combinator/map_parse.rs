use crate::parser::Parser;

pub trait MapParse<I, O> {
    fn map<T>(self, f: impl Fn(O) -> T) -> impl Parser<I, T>;
}

impl<I, O, P: Parser<I, O>> MapParse<I, O> for P {
    fn map<T>(self, f: impl Fn(O) -> T) -> impl Parser<I, T> {
        move |i| self(i).map(|(i, o)| (i, f(o)))
    }
}

#[cfg(test)]
mod tests {
    use super::MapParse;
    use crate::parser::{char, num, AndParse};

    #[test]
    fn test1() {
        let base = "1+1ans";
        let parser = num.and(char('+')).and(num).map(|((a, _), b)| a + b);
        assert_eq!(parser(base), Ok(("ans", 2)))
    }
}
