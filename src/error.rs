use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error<T> 
{
    ParseIntErrror(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    ParseCharError,
    Uninstalled,
    ParseError(T),
}

impl<T> fmt::Display for Error<T> 
where T: fmt::Debug + fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloatError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseIntErrror(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseCharError => write!(f, "ParseCharError"),
            Self::Uninstalled => write!(f, "未実装ですまない..."),
            Self::ParseError(s) => write!(f, "ParseError: {}", s),
        }
    }
}

impl<T> error::Error for Error<T>
where T: fmt::Debug + fmt::Display
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ParseFloatError(e) => Some(e),
            Self::ParseIntErrror(e) => Some(e),
            _ => None
        }
    }
}

impl<T> From<std::num::ParseFloatError> for Error<T> {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(e)
    }
}

impl<T> From<std::num::ParseIntError> for Error<T> {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseIntErrror(e)
    }
}   

pub type Result<T, E> = std::result::Result<T, Error<E>>;