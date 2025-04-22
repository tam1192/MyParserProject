use super::*;

/// [Substitute]の結果を保持する
///
/// [subパーサー][Substitute::sub]で結果を返すためのenumです。
#[derive(Debug, PartialEq)]
pub enum SubResult<A, B> {
    A(A),
    B(B),
}

/// パース失敗時に別の[Parser][crate::parser::Parser]で解析する
///
/// 戻り値に[Result]型を用いる[Parser][crate::parser::Parser]で使うことができます。
pub trait Substitute<I, A, AE> {
    /// メソッド呼び出し元(A)パーサーが失敗した時、メソッド引数(B)パーサーを試みるパーサーを作成します。
    ///
    /// ## 結果とエラー
    /// - メソッド呼び出し元(A)パーサーが[Ok]だった場合は、[SubResult::A]が返されます
    /// - メソッド呼び出し元(A)パーサーが[Err]だった場合は、メソッド引数(B)パーサーを使います
    /// - メソッド引数(B)パーサーが[Ok]だった場合は、[SubResult::B]が返されます
    /// - メソッド引数(B)パーサーが[Err]だった場合は、`(A, B)`のタプル形式でエラーが返されます
    ///
    /// | |A|B|
    /// |---|---|---|
    /// |解析順| 1 | 2 |
    /// |Ok| [SubResult::A] | [SubResult::B] |
    /// |Err| Bで解析する | `Err(AE, BE)` |
    ///
    /// ## Example
    /// ```
    /// use crate::parser::{base::{char, num}, combinator::*};
    ///
    /// let input = "*123";
    /// let parser = char('*').sub(num);
    /// let (_, result) = parser(input);
    ///
    /// assert_eq!(result, Ok(SubResult::A('*')))
    /// ```
    fn sub<B, BE>(
        self,
        p: impl Parser<I, Result<B, BE>>,
    ) -> impl Parser<I, Result<SubResult<A, B>, (AE, BE)>>;

    /// メソッド引数(B)パーサーのエラーを無視します。  
    /// `sub`とは、戻り値の仕様が異なり、このパーサーはエラーを返すことはありません。  
    ///
    /// ## 結果とエラー
    /// - メソッド呼び出し元(A)パーサーが[Ok]だった場合は、[SubResult::A]が返されます
    /// - メソッド呼び出し元(A)パーサーが[Err]だった場合は、メソッド引数(B)パーサーを使います
    /// - メソッド引数(B)パーサーが[Ok]だった場合は、[SubResult::B]が返されます。
    /// - メソッド引数(B)パーサーが[Err]だった場合も、[SubResult::B]が返されます。
    ///
    /// | |A|B|
    /// |---|---|---|
    /// |解析順| 1 | 2 |
    /// |Ok| [SubResult::A] | [SubResult::B] |
    /// |Err| Bで解析する | [SubResult::B] |
    ///
    /// ## `sub`との使い分け
    /// メソッド引数(B)パーサーの戻り値が[Result]出ない場合も使うことができます。
    /// そのため、[trimer][crate::parser::str_parser::trimer]などのパーサーが使用可能です。
    fn sub_uncheck<B>(self, p: impl Parser<I, B>) -> impl Parser<I, SubResult<A, B>>;
}

// 実装
impl<I, A, AE, P> Substitute<I, A, AE> for P
where
    P: Parser<I, Result<A, AE>>,
{
    fn sub<B, BE>(
        self,
        p: impl Parser<I, Result<B, BE>>,
    ) -> impl Parser<I, Result<SubResult<A, B>, (AE, BE)>> {
        move |i| match self(i) {
            (i, Ok(a)) => (i, Ok(SubResult::A(a))),
            (i, Err(ae)) => match p(i) {
                (i, Ok(b)) => (i, Ok(SubResult::B(b))),
                (i, Err(be)) => (i, Err((ae, be))),
            },
        }
    }

    fn sub_uncheck<B>(self, p: impl Parser<I, B>) -> impl Parser<I, SubResult<A, B>> {
        move |i| match self(i) {
            (i, Ok(a)) => (i, SubResult::A(a)),
            (i, Err(_)) => {
                let (i, b) = p(i);
                (i, SubResult::B(b))
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
        assert_eq!(result, Ok(SubResult::A('*')))
    }
    // Bのパーサーで成功する時
    #[test]
    fn sub_success_b() {
        let input = "+123";
        let parser = str_parser::char('*').sub(str_parser::char('+'));
        let (_, result) = parser(input);
        assert_eq!(result, Ok(SubResult::B('+')))
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
        assert_eq!(result, SubResult::A('*'))
    }
    // Bのパーサーで成功する時
    #[test]
    fn sub_uncheck_success_b() {
        let input = "+123";
        let parser = str_parser::char('*').sub_uncheck(str_parser::char('+'));
        let (_, result) = parser(input);
        assert_eq!(result, SubResult::B(Ok('+')))
    }
    // ABどちらもパース不可能な時
    #[test]
    fn sub_uncheck_failure() {
        let input = "a123";
        let parser = str_parser::char('*').sub_uncheck(str_parser::char('+'));
        let (_, result) = parser(input);
        assert!(matches!(result, SubResult::B(Err(_))))
    }
}
