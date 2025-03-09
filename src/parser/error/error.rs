use super::*;

/// [crate::parser]に含まれる各種パーサーから発生したエラーを収容する構造体
/// 
/// [crate::parser]以下に含まれるパーサー関数のエラーを管理します。  
/// [super::ErrorKind]にジャンル分けされたエラー原因が含まれます。  
/// 
#[derive(Debug, PartialEq)]
pub struct Error
{
    kind: ErrorKind,
}