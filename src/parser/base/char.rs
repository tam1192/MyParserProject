use super::*;

/// 先頭一文字でパースする
///
/// # Example
/// ```rust
/// use crate::parser::base::char;
///
/// let input = "*123";
/// let (rest, result) = char('*')(input);
/// assert_eq!(result, Ok('*'));
/// ```
///
/// # Error
/// kindが [crate::parser::ErrorKind::ParseCharError] になります。
///
pub fn char<'a>(c: char) -> impl Parser<&'a str, Result<char, Error>> {
    move |i: &'a str| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 正常系
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

    // 異常
    #[test]
    fn dissociation_test() {
        let base = "123*";
        let parser = char('*');
        let (rest, result) = parser(base);
        // 残った文字列
        assert_eq!(rest, "123*");
        // パースした文字（エラー）
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::ParseCharError);
    }
}
