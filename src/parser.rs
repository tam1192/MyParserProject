use crate::error::*;
pub trait Parser<I, O>: Fn(I) -> Result<(I, O), I> {}
impl <F, I, O> Parser<I, O> for F where F: Fn(I) -> Result<(I, O), I> {}

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

pub mod num;
pub mod num_ex;
pub mod char;

pub use num::space_trimer;
pub use num::num;
pub use num_ex::num_ex;
pub use char::char;