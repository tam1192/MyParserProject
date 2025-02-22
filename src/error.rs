use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseIntErrror(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    ParseCharError,
    Uninstalled,
    NumberPowError,
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloatError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseIntErrror(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseCharError => write!(f, "ParseCharError"),
            Self::Uninstalled => write!(f, "未実装ですまない..."),
            Self::NumberPowError => write!(f, "NumberPowError"),
        }
    }
}

impl error::Error for Error
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ParseFloatError(e) => Some(e),
            Self::ParseIntErrror(e) => Some(e),
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

pub type Result<T> = std::result::Result<T, Error>;
