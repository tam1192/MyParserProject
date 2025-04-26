use super::*;

pub fn not<I, R, P: Parser<I, R>>(p: P) -> impl Parser<I, Result<I, Error>> {
    move |i| todo!()
}
