use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseIntErrror(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    ParseCharError,
    CombinatorParseError(Box<Self>),
    Uninstalled,
    NumberError(crate::number::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloatError(parse_float_error) => write!(f, "{}", parse_float_error),
            Self::ParseIntErrror(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseCharError => write!(f, "ParseCharError"),
            Self::Uninstalled => write!(f, "未実装ですまない..."),
            Self::NumberError(e) => write!(f, "{}", e),
            Self::CombinatorParseError(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ParseFloatError(e) => Some(e),
            Self::ParseIntErrror(e) => Some(e),
            Self::NumberError(e) => Some(e),
            Self::CombinatorParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseIntErrror(e)
    }
}

impl From<crate::number::error::Error> for Error {
    fn from(e: crate::number::error::Error) -> Self {
        Error::NumberError(e)
    }
}
