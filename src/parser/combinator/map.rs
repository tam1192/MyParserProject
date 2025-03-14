use super::*;

/// パーサーの結果に関数を適用するメソッドを提供する
pub trait Map<I, O> {
    /// パーサーの結果に関数を適用
    ///
    /// ## 引数について
    /// パーサーが出力する結果がそのまま引数となります。  
    fn map<T>(self, f: impl Fn(O) -> T + Clone) -> impl Parser<I, T>;
}
