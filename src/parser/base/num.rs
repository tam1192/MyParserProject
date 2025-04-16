use super::*;
/// 数値でパースする
///
/// 数値をパースする [super::Parser] です。
///
/// - 引数の型は[std::str]です。  
/// - `0-9`の範囲でパースを行います。  
///   - `+`,`-`,`.`などの数学記号には対応しません。  
/// - 先頭から解析を行います。  
///
/// # 成功時
/// - 解析できた数値が返却されます。
/// # エラー時
/// - [super::error::Error]が返却されます。
///   - kindは [super::ErrorKind::ParseNumError] になります。
///   - sourceは [std::num::ParseIntError] になります。
///
/// # Example
/// ```rust
/// use crate::parser::base::num;
///
/// let input = "123abc";
/// let (rest, result) = num(input);
/// assert_eq!(result, Ok(123));
/// ```
///
///
pub fn num<'a>(i: &'a str) -> (&'a str, Result<i64, Error>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 正常系: 数字がパースできる場合
    #[test]
    fn success_test() {
        let input = "123abc";
        let (rest, result) = num(input);
        // 残った文字列
        assert_eq!(rest, "abc");
        // パースした数字
        assert_eq!(result, Ok(123));
    }

    // 異常系: 数字がパースできない場合
    #[test]
    fn dissociation_test() {
        let input = "abc123";
        let (rest, result) = num(input);
        // 残った文字列
        assert_eq!(rest, "abc123");
        // パースした数字（エラー）
        assert!(matches!(
            result.unwrap_err().kind(),
            &ErrorKind::ParseNumError(_)
        ));
    }
}
