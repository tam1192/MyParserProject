use super::*;

/// パーサーの結果に関数を適用するメソッドを提供する
pub trait Map<I, O> {
    /// パーサーの結果に関数を適用
    ///
    /// クロージャーを受け取り、mapメソッドを呼び出す手前までのパーサーが出力した結果に、関数を適用します。
    ///
    /// # ヒント
    /// - [.cat][concat::Concat::cat]メソッドは(`メソッド呼び出し元`, `メソッド引数`)形式で出力されます
    /// - [.sub][substitute::Substitute::sub]メソッドは[substitute::SubResult]で出力されます
    fn map<T>(self, f: impl Fn(O) -> T + Clone) -> impl Parser<I, T>;
}

impl<I, O, P> Map<I, O> for P
where
    P: Parser<I, O>,
{
    fn map<T>(self, f: impl Fn(O) -> T + Clone) -> impl Parser<I, T> {
        move |i| todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_2x_test() {
        // numパーサーで数値を取得し、2倍した結果を取得するテスト
        let base = "10x";
        let parser = str_parser::num.map(|x| x.map(|i| i * 2));
        let (i, r1) = parser(base);
        // パース結果
        assert_eq!(r1, Ok(4));
        // 余った部分
        assert_eq!(i, "x");
    }

    #[test]
    fn map_error_change_test() {
        // numパーサーのエラーを0に変換する
        let base = "x123";
        let parser = str_parser::num.map(|x| x.unwrap_or(0));
        let (i, r1) = parser(base);
        // パース結果
        assert_eq!(r1, 0);
        // 余った部分
        assert_eq!(i, "x123");
    }
}
