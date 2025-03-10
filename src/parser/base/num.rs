use super::*;
/// 先端から続くまで、0-9でパースする関数
/// 
/// # Example
/// ```rust
/// let input = "123abc";
/// let (rest, result) = num(input);
/// assert_eq!(result, Ok(123));
/// ```
/// 
/// # Error
/// - 数値がパースできなかった時
/// kindが [crate::parser::ErrorKind::ParseNumError] になります。
/// sourceは [std::num::ParseIntError] になります。
/// 
pub fn num<'a>(i: &'a str) -> (&'a str, Result<i64, Error>) {
    todo!()
}

#[cfg(test)]
mod tests{
    use std::num::ParseIntError;

    use super::*;

    // 正常系: 数字がパースできる場合
    #[test]
    fn success_test() {
        let input = "123abc";
        let (rest, result) = num(input);
        assert_eq!(result, Ok(123));
        assert_eq!(rest, "abc");
    }

    // 異常系: 数字がパースできない場合
    #[test]
    fn dissociation_test() {
        let input = "abc123";
        let (rest, result) = num(input);
        assert_eq!(rest, "abc123");
        assert!(matches!(result.unwrap_err().kind(), &ErrorKind::ParseNumError(_)));
    }

}