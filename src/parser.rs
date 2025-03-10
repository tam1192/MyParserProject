/// 単体でパーサーとして動作する、もっとも基本的なパーサー群。
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