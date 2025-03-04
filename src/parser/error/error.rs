/// [crate::parser]に含まれる各種パーサーから発生したエラーを収容する構造体
/// 
/// [crate::parser]以下に含まれるパーサー関数のエラーを管理します。  
/// [super::ErrorKind]にジャンル分けされたエラー原因が含まれます。  
/// 
/// # 比較時の制約
/// `PartialEq`トレイトを実装していますが、`source`フィールドは比較の対象外です。  
#[derive(Debug)]
pub struct Error {
    kind: super::ErrorKind,
    source: Box<dyn std::error::Error>,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}