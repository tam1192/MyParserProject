use super::*;

/// 文字でパースする
///
/// 引数に入れた文字をパースする [crate::parser::Parser] を返します。
///
/// # パーサーの仕様について
/// - 引数は[std::str]です。
/// - 引数の先頭で検査を行います。
/// ## 成功時
/// - 条件の文字が返却されます。
/// ## エラー時
/// - [super::error::Error]のkindが [super::ErrorKind::ParseCharError] になります。
///
/// # Example
/// ```rust
/// use crate::parser::base::char;
///
/// let input = "*123";
/// let (rest, result) = char('*')(input);
/// assert_eq!(result, Ok('*'));
/// ```
pub fn char<'a>(c: char) -> impl Parser<&'a str, Result<char, Error>> {
    move |i: &'a str| todo!()
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
