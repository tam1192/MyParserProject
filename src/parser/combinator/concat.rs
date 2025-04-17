use super::*;

/// パーサーを繋げるメソッドを提供する
///
/// [crate::parser::Parser] 型の関数オブジェクトを連結させるためのメソッドを提供します。

pub trait Concat<I, O1, O2> {
    /// パーサーを連結させ、両方の結果を返す
    ///
    /// # 結果について
    /// `(I, (O1, O2))`二重タプルで返します  
    /// - Iはパース後に残った部分
    /// - O1はメソッドを呼び出す元のパーサーが返した結果
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
    fn cat(self, p: impl Parser<I, O2>) -> impl Parser<I, (O1, O2)>;

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
    fn cat_a(self, p: impl Parser<I, O2>) -> impl Parser<I, O1>;

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
    fn cat_b(self, p: impl Parser<I, O2>) -> impl Parser<I, O2>;
}

// 実装
impl<I, O1, O2, P> Concat<I, O1, O2> for P
where
    P: Parser<I, O1>,
{
    fn cat(self, p: impl Parser<I, O2>) -> impl Parser<I, (O1, O2)> {
        move |i| todo!()
    }

    fn cat_a(self, p: impl Parser<I, O2>) -> impl Parser<I, O1> {
        move |i| todo!()
    }

    fn cat_b(self, p: impl Parser<I, O2>) -> impl Parser<I, O2> {
        move |i| todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cat成功時
    #[test]
    fn cat_success_test() {
        let base = "*123";
        let parser = str_parser::char('*').cat(str_parser::num);
        let (i, (r1, r2)) = parser(base);
        // パースした結果
        assert_eq!(r1, Ok('*'));
        assert_eq!(r2, Ok(123));
        // 取り残された文字
        assert_eq!(i, "");
    }

    // cat_a成功時
    #[test]
    fn cat_a_success_test() {
        let base = "*123";
        let parser = str_parser::char('*').cat_a(str_parser::num);
        let (i, r1) = parser(base);
        // パースした結果
        assert_eq!(r1, Ok('*'));
        // 取り残された文字
        assert_eq!(i, "");
    }

    // cat_b成功時
    #[test]
    fn cat_b_success_test() {
        let base = "*123";
        let parser = str_parser::char('*').cat_b(str_parser::num);
        let (i, r1) = parser(base);
        // パースした結果
        assert_eq!(r1, Ok(123));
        // 取り残された文字
        assert_eq!(i, "");
    }

    // エラー
    #[test]
    fn cat_error_test() {
        // パースしたい文字列の並び方が、パーサーの並び方と異なる
        let base = "+-abc";
        let parser = str_parser::char('-').cat(str_parser::char('+'));
        let (i, (r1, r2)) = parser(base);
        // パースした結果
        assert_eq!(r1.unwrap_err().kind(), &ErrorKind::ParseCharError);
        assert_eq!(r2.unwrap_err().kind(), &ErrorKind::ParseCharError);
        // 取り残された文字
        assert_eq!(i, "+-abc");
    }
}
