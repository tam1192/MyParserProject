use crate::parser::Parser;

pub trait Join<I, A> {
    fn join<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)>;
}

impl<I, A, T: Parser<I, A>> Join<I, A> for T{
    fn join<B>(self, parser: impl Parser<I, B>) -> impl Parser<I, (A, B)> {
        move |i| {
            self(i).and_then(|(i, o1)| {
                parser(i).map(|(i, o2)|{
                    (i,(o1, o2))
                })
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{char, num, trimer};
    use super::Join;

    #[test]
    fn test2() {
        let base = "123+abc";
        let parser = num.join(char('+'));
        assert_eq!(parser(base), Ok(("abc", (123, ()))))
    }

    #[test]
    fn test3() {
        let base = "   123+abc";
        let parser = trimer.join(num).join(char('+'));
        assert_eq!(parser(base), Ok(("abc", (((), 123), ()))))
    }
}
