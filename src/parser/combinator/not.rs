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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::str_parser::*;

    #[test]
    fn simple() {
        let input = "abc123";
        let parser = not(num);
        let (remain, result) = parser(input);
        assert_eq!(result, "abc");
        assert_eq!(remain, "123");
    }

    // 入力パーサーがすぐに成功しちゃう例
    #[test]
    fn zero_length_remain() {
        let input = "123";
        let parser = not(num);
        let (remain, result) = parser(input);
        assert_eq!(result, "123");
        assert_eq!(remain, "");
    }

    // 入力パーサーがすべて失敗する例
    #[test]
    fn input_parser_always_fail() {
        let input = "abc";
        let parser = not(num);
        let (remain, result) = parser(input);
        assert_eq!(result, "");
        assert_eq!(remain, "abc");
    }
}
