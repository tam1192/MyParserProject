use super::*;

/// 文字でパースする
///
/// 引数に入れた文字を条件に解析する [Parser][super::Parser] を**作成**します。
///
/// # パーサーの仕様について
/// - 先頭のみ解析を行います。
/// ## 成功時
/// - 条件の文字が解析できたら、その文字が結果として返されます。
/// ## エラー時
/// - [エラー][super::error::Error]が返却されます。
///   - [kind][super::error::Error::kind]が [ParseCharError][super::ErrorKind::ParseCharError] になります。
///
/// # Example
/// ```rust
/// use my_parser_project::parser::str_parser::char;
///
/// let input = "*123";
/// let (rest, result) = char('*')(input);
/// assert_eq!(result, Ok('*'));
/// ```
pub fn char<'a>(c: char) -> impl Parser<&'a str, Result<char, Error>> {
    move |i: &'a str| {
        if i.chars().next() == Some(c) {
            (&i[1..], Ok(c))
        } else {
            (&i, Err(Error::new(ErrorKind::ParseCharError)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 正常系: パースできる場合
    #[test]
    fn success_test() {
        let base = "*123";
        let parser = char('*');
        let (rest, result) = parser(base);
        // 残った文字列
        assert_eq!(rest, "123");
        // パースした文字
        assert_eq!(result, Ok('*'));
    }

    // 異常系: 文字が異なりパースできない場合
    #[test]
    fn dissociation_test() {
        // パースする文字の手前に、別の文字があるためにパース不可能
        let base = "123*";
        let parser = char('*');
        let (rest, result) = parser(base);
        // 残った文字列
        assert_eq!(rest, "123*");
        // パースした文字（エラー）
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::ParseCharError);
    }
}
