use crate::parser::Parser;
use crate::error::Result;

trait Join<I, O1, O2> {
    fn join(&self, parser: impl Parser<I, O1>) -> Box<impl Parser<I, (O1, O2)>>;
}

impl<I, O1, O2> Join<I, O1, O2> for dyn Parser<I, O1> {
    fn join(&self, parser: impl Parser<I, O2>) -> Box<impl Parser<I, (O1, O2)>> {
        Box::new(move |i| {
            self(i).and_then(|(i, o1)| {
                parser(i).map(|(i, o2)|{
                    (i, (o1, o2))
                })
            })
        })
    }
}

pub fn join<I, A, B>(parser1: impl Parser<I,A>, parser2: impl Parser<I,B>) -> impl Parser<I, (A, B)> {
    move |i| {
        parser1(i).and_then(|(i, out1)| {
            parser2(i).map(|(i, out2)| {
                (i, (out1, out2))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::join;
    use crate::parser::{num, char};

    #[test]
    fn test1() {
        let base = "123+abc";
        let parser = join(num, char('+'));
        assert_eq!(parser(base), Ok(("abc", (123, ()))))
    }
}