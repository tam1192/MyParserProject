use crate::error::*;
pub trait Parser<I, O>: Fn(I) -> Result<(I, O), I> {}
impl<F, I, O> Parser<I, O> for F where F: Fn(I) -> Result<(I, O), I> {}

mod base;
pub use base::*;

mod combinator;
pub use combinator::*;

mod template;
pub use template::*;
