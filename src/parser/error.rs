use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Error {
    // 数値パーサー用エラー
    ParseIntErrror(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    // 文字列エラー
    ParseCharError,
    // コンビネーターエラー
    // andでは1以上、orでは2つ子を持つ
    CombinatorParseError(Box<Self>, Option<Box<Self>>),
    // Numberのエラーを持つエラー
    NumberError(crate::number::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloatError(parse_float_error) => write!(f, "{}", parse_float_error),
            Self::ParseIntErrror(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseCharError => write!(f, "ParseCharError"),
            Self::NumberError(e) => write!(f, "{}", e),
            Self::CombinatorParseError(a, b) => {
                let s = format!("CombinatorParseError:\nA:\n{}", a);
                if let Some(b) = b {
                    write!(f, "{}\nB:\n{}", s, b)
                } else {
                    write!(f, "{}", s)
                }
            }
            _ => write!(f, ""),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ParseFloatError(e) => Some(e),
            Self::ParseIntErrror(e) => Some(e),
            Self::NumberError(e) => Some(e),
            Self::CombinatorParseError(a, None) => Some(a),
            // 二つある場合は取り出せない
            // division_combinator_parse_error関数でエラーを分裂させる
            Self::CombinatorParseError(_, Some(_)) => None,
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

impl Error {
    fn division_combinator_parse_error(self) -> Option<(Self, Self)> {
        if let Self::CombinatorParseError(a, Some(b)) = self {
            Some((
                Self::CombinatorParseError(a, None),
                Self::CombinatorParseError(b, None),
            ))
        } else {
            None
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
