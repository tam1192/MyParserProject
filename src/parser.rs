pub trait Parser<I, O>: ParserOnce<I, O> + Clone {}
impl<F, I, O> Parser<I, O> for F where F: ParserOnce<I, O> + Clone {}

pub trait ParserOnce<I, O>: Fn(I) -> error::Result<(I, O)> {}
impl<F, I, O> ParserOnce<I, O> for F where F: Fn(I) -> error::Result<(I, O)> {}

/// Single usable parsers
pub mod base;

/// Parser to combine parsers
pub mod combinator;

/// Commonly used parsers
mod template;
pub use template::*;

mod error;
pub use error::Error;
pub use error::Result;
