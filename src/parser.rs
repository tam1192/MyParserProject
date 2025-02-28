pub trait Parser<I, O, E>: ParserOnce<I, O, E> + Clone {}
impl<F, I, O, E> Parser<I, O, E> for F where F: ParserOnce<I, O, E> + Clone {}

pub trait ParserOnce<I, O, E>: Fn(I) -> Result<(I, O), E> {}
impl<F, I, O, E> ParserOnce<I, O, E> for F where F: Fn(I) -> Result<(I, O), E> {}

mod base;
pub use base::*;

mod combinator;
pub use combinator::*;

mod template;
pub use template::*;

pub mod error;
