use std::ops::Add;

use crate::error::*;
pub trait Parser<I, O>: Fn(I) -> Result<(I, O), I> {}
impl<F, I, O> Parser<I, O> for F where F: Fn(I) -> Result<(I, O), I> {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Int(i64),
    Float(f64),
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
        match self {
            Number::Int(x) => {
                let y = match rhs {
                    Self::Int(y) => return Self::Int(x + y),
                    Number::Float(y) => y,
                };
                let x = x as f64;
                Self::Float(x + y)
            },
            Number::Float(x) => {
                let y = match rhs {
                    Number::Int(y) => y as f64,
                    Number::Float(y) => y,
                };
                Self::Float(x + y)
            },
        }
    }
}

mod base;
pub use base::*;

mod combinator;
pub use combinator::*;

mod template;
pub use template::*;
