use super::*;

/// [crate::parser]に含まれる各種パーサーから発生したエラーを収容する構造体
///
/// [crate::parser]以下に含まれるパーサー関数のエラーを管理します。  
/// [super::ErrorKind]にジャンル分けされたエラー原因が含まれます。  
///
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// エラーを作成
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
    /// ErrorKindを取得する
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::ParseNumError(parse_int_error) => {
                write!(f, "failed parse to number\n{}", parse_int_error)
            }
            ErrorKind::ParseCharError => write!(f, "failed parse to char"),
            ErrorKind::ParseStringError => write!(f, "failed parse to string"),
            ErrorKind::ParseNotError => write!(f, "failed parse to not"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::ParseNumError(parse_int_error) => Some(parse_int_error),
            _ => None,
        }
    }
}
