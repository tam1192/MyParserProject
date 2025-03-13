use super::*;

/// パーサーを繋げるメソッドを提供する
///
/// [crate::parser::Parser] 型の関数オブジェクトを連結させるためのメソッドを提供します。

trait Concat<I, O1, O2> {
    /// パーサーを連結させ、両方の結果を返す
    ///
    /// # 結果について
    /// `(I, (O1, O2))`二重タプルで返します
    /// - Iはパース後に残った部分 /// - O1はメソッドを呼び出す元のパーサーが返した結果
    /// - O2はメソッドの引数に含めたパーサーが返した結果
    ///
    /// # Error
    /// 実行したパーサーにエラーが含まれる場合も、そのまま結果を出す
    ///
    /// # Example
    /// ```
    /// use crate::parser::{base::{char, num}, combinator::Concat};
    ///
    /// let input = "*123";
    /// let parser = char('*').cat(num);
    /// let (_, result) = parser(input);
    ///
    /// assert_eq!(result, (Ok('*'), Ok(123)))
    /// ```
    ///
    fn cat(self, p: impl Parser<I, O1>) -> impl Parser<I, (O1, O2)>;

    /// パーサーを連結させ、メソッド呼び出し元のパーサーが返した結果を返す
    ///
    /// 結果を必要としないパーサーを組み合わせるときに便利です
    /// # 結果について
    /// `(I, O1)`タプルで返します。
    /// - Iはパース後に残った部分
    /// - O1はメソッドを呼び出す元のパーサーが返した結果
    ///
    /// # Error
    /// 実行したパーサーにエラーが含まれる場合も、そのまま結果を出す
    ///
    /// # Example
    /// ```
    /// use crate::parser::{base::{char, num}, combinator::Concat};
    ///
    /// let input = "*123";
    /// let parser = char('*').cat_a(num);
    /// let (_, result) = parser(input);
    ///
    /// assert_eq!(result, Ok('*'))
    /// ```
    ///
    fn cat_a(self, p: impl Parser<I, O1>) -> impl Parser<I, (O1, O2)>;

    /// パーサーを連結させ、メソッド引数に含めたパーサーが返した結果を返す
    ///
    /// 結果を必要としないパーサーを組み合わせるときに便利です
    /// # 結果について
    /// `(I, O2)`二重タプルで返します
    /// - Iはパース後に残った部分
    /// - O2はメソッドの引数に含めたパーサーが返した結果
    ///
    /// # Error
    /// 実行したパーサーにエラーが含まれる場合も、そのまま結果を出す
    ///
    /// # Example
    /// ```
    /// use crate::parser::{base::{char, num}, combinator::Concat};
    ///
    /// let input = "*123";
    /// let parser = char('*').cat_b(num);
    /// let (_, result) = parser(input);
    ///
    /// assert_eq!(result, Ok(123))
    /// ```
    ///
    fn cat_b(self, p: impl Parser<I, O1>) -> impl Parser<I, (O1, O2)>;
}
