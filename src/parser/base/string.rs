use super::*;

/// 文字列でパースする
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
/// 
/// # Error
/// kindが [crate::parser::ErrorKind::ParseStringError] になります。
/// 
pub fn string<'a>(s: String) -> impl Parser<&'a str, Result<String, Error>> {
    move |i: &'a str| {
        todo!()
    }
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

    // 異常
    #[test]
    fn dissociation_test() {
        let base = "3+=";
        let parser = string("+=".to_string());
        let (rest, result) = parser(base);
        // 残った文字列
        assert_eq!(rest, "3+=");
        // パースした文字列（エラー）
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::ParseStringError);
    }
}