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
    /// ErrorKindを取得する
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
