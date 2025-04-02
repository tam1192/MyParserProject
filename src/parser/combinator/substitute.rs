use super::*;

/// [Sub]の結果を保持する
#[derive(PartialEq)]
pub enum SubResult<A, B> {
    A(A),
    B(B),
}

/// パース失敗時に別のパーサーを試すためのメソッドを提供する
///
/// 失敗する可能性のある [crate::parser::Parser] 型の関数オブジェクトに適用され、  
/// メソッド呼び出し元のパーサーが失敗した場合、引数のパーサーを試みます。
pub trait Substitute<I, A, AE> {
    /// 別のパーサーを試す
    ///
    /// # Errorと結果について
    /// `(I, Result<SubResult<A, B> (AE, BE)>)`のタプルで返します  
    /// - Iはパース後に残った部分
    ///
    /// ## ResultがOkの場合
    /// [SubResult]<A, B>で返されます
    /// - Aはメソッドを呼び出す元のパーサーが返した結果
    /// - Bはメソッドの引数に含めたパーサーが返した結果
    ///
    /// ## ResultがErrの場合  
    /// (AE, BE)のタプルで返されます  
    /// エラーは両方返されます  
    /// - Aはメソッドを呼び出す元のパーサーが返したエラー
    /// - Bはメソッドの引数に含めたパーサーが返したエラー
    ///
    ///
    /// # Example
    /// ```
    /// use crate::parser::{base::{char, num}, combinator::*};
    ///
    /// let input = "*123";
    /// let parser = char('*').sub(num);
    /// let (_, result) = parser(input);
    ///
    /// assert_eq!(result, Ok(SubResult::A('*')))
    /// ```
    ///
    fn sub<B, BE>(
        self,
        p: impl Parser<I, Result<B, BE>>,
    ) -> impl Parser<I, Result<SubResult<A, B>, (AE, BE)>>;
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
        move |i| todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 成功例
    #[test]
    fn sub_success() {
        let input = "*123";
        let parser = base::char('*').sub(base::num);
        let (_, result) = parser(input);
        assert_eq!(result, Ok(SubResult::A('*')))
    }
    #[test]
    fn sub_failure() {
        let input = "a123";
        let parser = char('*').sub(num);
        let (_, result) = parser(input);
        assert_eq!(result, Err((SubError::AE, SubError::BE)))
    }
}
