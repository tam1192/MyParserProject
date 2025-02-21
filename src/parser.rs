use crate::error::*;
pub trait ParserOnce<I, O>: Fn(I) -> Result<(I, O), I> {}
impl<F, I, O> ParserOnce<I, O> for F where F: Fn(I) -> Result<(I, O), I> {}

pub trait Parser<I, O>: ParserOnce<I, O> + Clone {}
impl<F, I, O> Parser<I, O> for F where F :Parser<I, O> + Clone {}

mod base;
pub use base::*;

mod combinator;
pub use combinator::*;

mod template;
pub use template::*;
