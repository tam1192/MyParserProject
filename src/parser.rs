use crate::error::*;
pub trait Parser<I, O>: Fn(I) -> Result<(I, O), I> {}
impl <F, I, O> Parser<I, O> for F where F: Fn(I) -> Result<(I, O), I> {}

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

mod base;
mod combinator;

pub use base::char::char;
pub use base::num::num;
pub use base::num_ex::num_ex;
pub use base::trimer::trimer;

pub use combinator::join::join;