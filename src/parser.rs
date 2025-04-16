/// 単体で動作する、基本的なパーサーです。
pub mod base;

/// 複数のパーサーを組み合わせて動作するパーサー
pub mod combinator;

// エラー
mod error;
pub use error::*;

/// スライスを先頭からパースして、結果を返す関数トレイトを提供します。
///
/// 引数に指定されたスライスの先頭から、指定された条件で検査をし、一致した場合は結果を返却します。
/// - [crate::parser::base] : 基本的なパーサーを提供します。
/// - [crate::parser::combinator] : 複雑な条件にも対応できるよう、パーサー同士を組み合わせることができるトレイトを提供します。
pub trait Parser<I, R>: Fn(I) -> (I, R) + Clone {}
// Fn(I) -> (I, Result<T,E>) を Parser<I, T, E> として使えるようにする
impl<I, R, F> Parser<I, R> for F where F: Fn(I) -> (I, R) + Clone {}
