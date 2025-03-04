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
/// kindが [crate::parser::ErrorKind::ParseNotNum] になります。
/// sourceは [std::num::ParseIntError] になります。
/// 
pub fn num<'a>(i: &'a str) -> (&'a str, Result<i64, crate::parser::Error>) {
    todo!()
}

#[cfg(test)]
mod tests{
    use super::*;

    // 正常系: 数字がパースできる場合
    fn success_test() {
        let input = "123abc";
        let (rest, result) = num(input);
        assert_eq!(result, Ok(123));
        assert_eq!(rest, "abc");
    }

    // 異常系: 数字がパースできない場合
    fn dissociation_test() {
        let input = "abc123";
        let (rest, result) = num(input);
        assert!(matches!(result, Err(_)));
        assert_eq!(rest, "abc123");
    }

}