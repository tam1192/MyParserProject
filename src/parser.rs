use std::ops::Add;

use crate::error::*;
pub trait Parser<I, O>: Fn(I) -> Result<(I, O), I> {}
impl<F, I, O> Parser<I, O> for F where F: Fn(I) -> Result<(I, O), I> {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        match value {
            Number::Int(x) => x as f64,
            Number::Float(x) => x,
        }
    }
}

impl From<Number> for i64 {
    fn from(value: Number) -> Self {
        match value {
            Number::Int(x) => x,
            Number::Float(x) => x as i64,
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x+y),
            _ => {
                let x = f64::from(self);
                let y = f64::from(self);
                Number::Float(x+y)
            }
        }
    }
}

mod base;
pub use base::*;

mod combinator;
pub use combinator::*;

mod template;
pub use template::*;
