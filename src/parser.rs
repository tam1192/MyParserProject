/// 単体でパーサーとして動作する、もっとも基本的なパーサー群。  
///
/// baseパーサーは、入力された文字列を左側からパースします。
/// ## 成功時
/// パースに成功したら、適切な型に変換されて戻されます。
/// ## 失敗時
/// パースに失敗したら、[crate::parser::Error] が返されます。
/// ## 共通
/// パースされなかった文字列は、パースされた文字列を除いた残りの文字列として返されます。
pub mod base;

/// 複数のパーサーを組み合わせて動作するパーサー
pub mod combinator;

// エラー
mod error;
pub use error::*;

/// パーサー関数をトレイトオブジェクト化
pub trait Parser<I, R>: Fn(I) -> (I, R) + Clone {}
// Fn(I) -> (I, Result<T,E>) を Parser<I, T, E> として使えるようにする
impl<I, R, F> Parser<I, R> for F where F: Fn(I) -> (I, R) + Clone {}
