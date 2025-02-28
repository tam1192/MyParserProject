use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {
    NumberPowError,
    NumberZeroDivError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumberPowError => write!(f, "NumberPowError"),
            Self::NumberZeroDivError => write!(f, "NumberZeroDivError"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
