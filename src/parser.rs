pub trait Parser<I, O>: ParserOnce<I, O> + Clone {}
impl<F, I, O> Parser<I, O> for F where F: ParserOnce<I, O> + Clone {}

pub trait ParserOnce<I, O>: Fn(I) -> error::Result<(I, O)> {}
impl<F, I, O> ParserOnce<I, O> for F where F: Fn(I) -> error::Result<(I, O)> {}

pub mod base;

pub mod combinator;

mod template;
pub use template::*;

mod error;
pub use error::Error;
pub use error::Result;
