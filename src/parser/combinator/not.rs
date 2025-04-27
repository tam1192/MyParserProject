use super::*;

/// [Parser]を成功させるまでに使った入力を取る
///
/// 戻り値が[Result]の[Parser]を、成功するまで入力をずらしながら試し、ずらした分の入力を結果として返します。
///
/// # Example
/// ```
/// use my_parser_project::parser::{combinator::*, str_parser::*};
///  
/// let base = "abc123"
/// let parser = not(num);
/// let (remain, result) = parser(base);
///
/// assert_eq!(result, "abc");
/// assert_eq!(remain, "123");
/// ```
///
pub fn not<I, T, E, P: Parser<I, Result<T, E>>>(p: P) -> impl Parser<I, I> {
    move |i| todo!()
}
