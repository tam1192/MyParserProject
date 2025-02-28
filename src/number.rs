use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::parser::error::*;

/// Number allows integers and floats to be managed as enums
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{}", i),
            Number::Float(i) => write!(f, "{}", i),
        }
    }
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
            (Number::Int(x), Number::Int(y)) => Number::Int(x + y),
            _ => {
                let x = f64::from(self);
                let y = f64::from(rhs);
                Number::Float(x + y)
            }
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x - y),
            _ => {
                let x = f64::from(self);
                let y = f64::from(rhs);
                Number::Float(x - y)
            }
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x * y),
            _ => {
                let x = f64::from(self);
                let y = f64::from(rhs);
                Number::Float(x * y)
            }
        }
    }
}

impl Div for Number {
    type Output = Result<Number>;

    fn div(self, rhs: Self) -> Self::Output {
        let x = f64::from(self);
        let y = f64::from(rhs);
        if y != 0.0 {
            Ok(Number::Float(x / y))
        } else {
            Err(Error::NumberPowError)
        }
    }
}

impl Number {
    pub fn pow(self, rhs: Self) -> Result<Number> {
        match (self, rhs) {
            (Number::Int(x), Number::Int(y)) => {
                let x = x;
                let y = y;
                if y >= 0 {
                    let x = x.pow(y as u32);
                    return Ok(Number::Int(x));
                }
            }
            _ => {
                let x = f64::from(self);
                let y = f64::from(rhs);
                if y >= 0.0 {
                    let x = x.powf(y);
                    return Ok(Number::Float(x));
                }
            }
        }
        Err(Error::NumberPowError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = Number::Int(1);
        let y = Number::Int(1);
        assert_eq!(x + y, Number::Int(2))
    }

    #[test]
    fn test2() {
        let x = Number::Int(2);
        let y = Number::Int(1);
        assert_eq!(x - y, Number::Int(1))
    }

    #[test]
    fn test3() {
        let x = Number::Int(2);
        let y = Number::Int(3);
        assert_eq!(x * y, Number::Int(6))
    }

    #[test]
    fn test4() {
        let x = Number::Int(2);
        let y = Number::Int(2);
        assert_eq!(x / y, Ok(Number::Float(1.0)))
    }

    #[test]
    fn test5() {
        let x = Number::Int(1);
        let y = Number::Float(1.0);
        assert_eq!(x + y, Number::Float(2.0))
    }

    #[test]
    fn test6() {
        let x = Number::Float(2.0);
        let y = Number::Int(1);
        assert_eq!(x - y, Number::Float(1.0))
    }

    #[test]
    fn test7() {
        let x = Number::Float(2.0);
        let y = Number::Float(3.0);
        assert_eq!(x * y, Number::Float(6.0))
    }
}
