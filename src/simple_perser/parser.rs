use super::error::*;

/// Parser trait
pub trait Parser<I, O>: Fn(I) -> Result<(I, O)> {}
impl <F, I, O> Parser<I, O> for F where F: Fn(I) -> Result<(I, O)> {}