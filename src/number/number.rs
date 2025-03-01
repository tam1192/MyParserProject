use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use super::error::{Error, ErrorKind};

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
        // 型を合わせて足し算を行う
        // Floatを優先にする
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
        // 型を合わせて引き算を行う
        // Floatを優先にする
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
        // 型を合わせて掛け算を行う
        // Floatを優先にする
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
    type Output = Result<Number, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        // 計算結果が出るまで整数か小数か判断が難しいので
        // 全て小数で返却する
        let x = f64::from(self);
        let y = f64::from(rhs);
        if y != 0.0 {
            Ok(Number::Float(x / y))
        } else {
            Err(Error::from(ErrorKind::ZeroDiv))
        }
    }
}

impl Number {
    pub fn pow(self, rhs: Self) -> Number {
        // 計算結果が出るまで整数か小数か判断が難しいので
        // 全て小数で返却する
        let x = f64::from(self);
        let y = f64::from(rhs);
        // powfが一番早い
        // 参照: https://qiita.com/tatsuya6502/items/d50e4b131130aa5b5ab6
        let x = x.powf(y);
        Number::Float(x)
    }
}
