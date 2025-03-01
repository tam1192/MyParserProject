use std::{error, fmt};

// Errorを構造体(struct)と列挙型(enum)に分けるのは、Enumが保有するデータを隠蔽できないため
// 参考: https://qiita.com/MasashiSUZUKI/items/b09881839c4e02b2f485

/// A list specifying general categories of Number type error.
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
/// It is used with the [Error] type.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    /// division-by-zero error
    ZeroDiv,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDiv => write!(f, "NumberZeroDivError"),
        }
    }
}

/// The error type of [super::Number] type.
/// Handles exceptions that occur in the calculation process.
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // sourceはErrorKindに含まれる
        match self.kind {
            _ => None,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Self { kind: value }
    }
}
