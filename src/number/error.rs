use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Pow,
    ZeroDiv,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pow => write!(f, "NumberPowError"),
            Self::ZeroDiv => write!(f, "NumberZeroDivError"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Self { kind: value }
    }
}
