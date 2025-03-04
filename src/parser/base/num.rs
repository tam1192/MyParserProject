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