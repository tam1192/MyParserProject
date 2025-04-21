use super::*;
/// 数値でパースする
///
/// 数値を条件に解析する [Parser][super::Parser] です。
///
/// - `0-9`の範囲でパースを行います。  
///   - `+`,`-`,`.`などの数学記号には対応しません。  
/// - 先頭から解析を行います。  
///
/// # 成功時
/// - 数値が解析できたら、[`parse<u64>`][`str::parse<u64>`]によって変換された値が、結果として返されます。
/// # エラー時
/// - 解析できなかった場合、[エラー][super::error::Error]が返されます。
///   - [kind][super::error::Error::kind]は [ParseNumError][super::ErrorKind::ParseNumError] になります。
///   - [source][std::error::Error::source]は[`parse<u64>`][`str::parse<u64>`()] のエラーである [ParseIntError][std::num::ParseIntError] になります。
///
/// # Example
/// ```rust
/// use crate::parser::str_parser::num;
///
/// let input = "123abc";
/// let (rest, result) = num(input);
/// assert_eq!(result, Ok(123));
/// ```
///
///
pub fn num<'a>(i: &'a str) -> (&'a str, Result<u64, Error>) {
    let l = i.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    match i[..l].parse::<u64>() {
        Ok(n) => (&i[l..], Ok(n)),
        Err(e) => (i, Err(Error::new(ErrorKind::ParseNumError(e)))),
    }
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
