use super::*;

/// 文字列でパースする
///
/// 引数に入れた文字列を条件に解析する [super::Parser] を**作成**します。
///
/// # パーサーの仕様について
/// - 先頭から解析を行います。
/// ## 成功時
/// - 条件の文字列が解析できたら、その文字列が結果として返されます。
/// ## エラー時
/// - [エラー][super::error::Error]が返却されます。
///   - [kind][super::error::Error::kind]が [ParseStringError][super::ErrorKind::ParseStringError] になります。
///
/// # Example
/// ```rust
/// use crate::parser::base::string;
///
/// let input = "hello_world";
/// let (rest, result) = string("hello".to_string())(input);
/// assert_eq!(result, Ok("hello"));
/// assert_eq!(rest, "_world");
/// ```
pub fn string<'a>(s: String) -> impl Parser<&'a str, Result<String, Error>> {
    move |i: &'a str| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 正常系
    #[test]
    fn success_test() {
        let base = "+=3";
        let parser = string("+=".to_string());
        let (rest, result) = parser(base);
        // 残った文字列
        assert_eq!(rest, "3");
        // パースした文字列
        assert_eq!(result, Ok("+=".to_string()));
    }

    // 異常系：文字列が異なりパースできない
    #[test]
    fn dissociation_test() {
        // 本来パースしたい文字列の手前に、別の文字が含まれている
        let base = "3+=";
        let parser = string("+=".to_string());
        let (rest, result) = parser(base);
        // 残った文字列
        assert_eq!(rest, "3+=");
        // パースした文字列（エラー）
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::ParseStringError);
    }
}
