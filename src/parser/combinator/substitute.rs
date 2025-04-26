use super::*;

/// [Substitute]の結果を保持する
///
/// [subパーサー][Substitute::sub]で結果を返すためのenumです。
#[derive(Debug, PartialEq)]
pub enum SubResult<Main, Sub> {
    Main(Main),
    Sub(Sub),
}

/// パース失敗時に別の[Parser][crate::parser::Parser]で解析する
///
/// 戻り値に[Result]型を用いる[Parser][crate::parser::Parser]で使うことができます。
pub trait Substitute<I, Main, MainE> {
    /// メソッド呼び出し元(Main)パーサーが失敗した時、メソッド引数(B)パーサーを試みるパーサーを作成します。
    ///
    /// ## 結果とエラー
    /// - メソッド呼び出し元(Main)パーサーが[Ok]だった場合は、[SubResult::Main]が返されます
    /// - メソッド呼び出し元(Main)パーサーが[Err]だった場合は、メソッド引数(Sub)パーサーを使います
    /// - メソッド引数(Sub)パーサーが[Ok]だった場合は、[SubResult::Sub]が返されます
    /// - メソッド引数(Sub)パーサーが[Err]だった場合は、`(Main, Sub)`のタプル形式でエラーが返されます
    ///
    /// | |A|B|
    /// |---|---|---|
    /// |解析順| 1 | 2 |
    /// |Ok| [SubResult::Main] | [SubResult::Sub] |
    /// |Err| Subで解析する | `Err(MainE, SubE)` |
    ///
    /// ## Example
    /// ```
    /// use my_parser_project::parser::{str_parser::{char, num}, combinator::*};
    ///
    /// let input = "*123";
    /// let parser = char('*').sub(num);
    /// let (_, result) = parser(input);
    ///
    /// assert_eq!(result, Ok(SubResult::Main('*')))
    /// ```
    fn sub<Sub, SubE>(
        self,
        p: impl Parser<I, Result<Sub, SubE>>,
    ) -> impl Parser<I, Result<SubResult<Main, Sub>, (MainE, SubE)>>;

    /// メソッド引数(Sub)パーサーのエラーを無視します。  
    /// `sub`とは、戻り値の仕様が異なり、このパーサーはエラーを返すことはありません。  
    ///
    /// ## 結果とエラー
    /// - メソッド呼び出し元(Main)パーサーが[Ok]だった場合は、[SubResult::Main]が返されます
    /// - メソッド呼び出し元(Main)パーサーが[Err]だった場合は、メソッド引数(Sub)パーサーを使います
    /// - メソッド引数(Sub)パーサーが[Ok]だった場合は、[SubResult::Sub]が返されます。
    /// - メソッド引数(Sub)パーサーが[Err]だった場合も、[SubResult::Sub]が返されます。
    ///
    /// | |Main|Sub|
    /// |---|---|---|
    /// |解析順| 1 | 2 |
    /// |Ok| [SubResult::Main] | [SubResult::Sub] |
    /// |Err| Subで解析する | [SubResult::Sub] |
    ///
    /// ## `sub`との使い分け
    /// メソッド引数(Sub)パーサーの戻り値が[Result]出ない場合も使うことができます。
    /// そのため、[trimer][crate::parser::str_parser::trimer]などのパーサーが使用可能です。
    fn sub_uncheck<Sub>(self, p: impl Parser<I, Sub>) -> impl Parser<I, SubResult<Main, Sub>>;
}

// 実装
impl<I, Main, MainE, P> Substitute<I, Main, MainE> for P
where
    P: Parser<I, Result<Main, MainE>>,
{
    fn sub<Sub, SubE>(
        self,
        p: impl Parser<I, Result<Sub, SubE>>,
    ) -> impl Parser<I, Result<SubResult<Main, Sub>, (MainE, SubE)>> {
        move |i| match self(i) {
            (i, Ok(a)) => (i, Ok(SubResult::Main(a))),
            (i, Err(ae)) => match p(i) {
                (i, Ok(b)) => (i, Ok(SubResult::Sub(b))),
                (i, Err(be)) => (i, Err((ae, be))),
            },
        }
    }

    fn sub_uncheck<B>(self, p: impl Parser<I, B>) -> impl Parser<I, SubResult<Main, B>> {
        move |i| match self(i) {
            (i, Ok(a)) => (i, SubResult::Main(a)),
            (i, Err(_)) => {
                let (i, b) = p(i);
                (i, SubResult::Sub(b))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A:メソッド呼び出し元
    // B:メソッド引数

    // Aのパーサーで成功する時
    #[test]
    fn sub_success_a() {
        let input = "*123";
        let parser = str_parser::char('*').sub(str_parser::char('+'));
        let (_, result) = parser(input);
        assert_eq!(result, Ok(SubResult::Main('*')))
    }
    // Bのパーサーで成功する時
    #[test]
    fn sub_success_b() {
        let input = "+123";
        let parser = str_parser::char('*').sub(str_parser::char('+'));
        let (_, result) = parser(input);
        assert_eq!(result, Ok(SubResult::Sub('+')))
    }
    // ABどちらもパース不可能な時
    #[test]
    fn sub_failure() {
        let input = "a123";
        let parser = str_parser::char('*').sub(str_parser::char('+'));
        let (_, result) = parser(input);
        assert!(matches!(result, Err(_)))
    }

    // Aのパーサーで成功する時
    #[test]
    fn sub_uncheck_success_a() {
        let input = "*123";
        let parser = str_parser::char('*').sub_uncheck(str_parser::char('+'));
        let (_, result) = parser(input);
        assert_eq!(result, SubResult::Main('*'))
    }
    // Bのパーサーで成功する時
    #[test]
    fn sub_uncheck_success_b() {
        let input = "+123";
        let parser = str_parser::char('*').sub_uncheck(str_parser::char('+'));
        let (_, result) = parser(input);
        assert_eq!(result, SubResult::Sub(Ok('+')))
    }
    // ABどちらもパース不可能な時
    #[test]
    fn sub_uncheck_failure() {
        let input = "a123";
        let parser = str_parser::char('*').sub_uncheck(str_parser::char('+'));
        let (_, result) = parser(input);
        assert!(matches!(result, SubResult::Sub(Err(_))))
    }
}
