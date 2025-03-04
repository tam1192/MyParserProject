/// 単体でパーサーとして動作する、もっとも基本的なパーサー群。
pub mod base;

/// 複数のパーサーを組み合わせて動作するパーサー
pub mod combinator;

// エラー
mod error;
use error::*;

/// パーサー関数をトレイトオブジェクト化
trait Parser<I, T, E>: Fn(I) -> (I, Result<T, E>) + Clone {}
// Fn(I) -> (I, Result<T,E>) を Parser<I, T, E> として使えるようにする
impl<I, T, E, F> Parser<I, T, E> for F where F: Fn(I) -> (I, Result<T, E>) + Clone {}