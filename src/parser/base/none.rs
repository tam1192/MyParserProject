use super::*;

/// パースしないパーサー
///
/// # Example
/// ```rust
/// use crate::parser::base::none;
///
/// let input = "   *abc";
/// let (rest, _) = none(input);
/// assert_eq!(rest, "   *abc");
/// ```
///
/// # Error
/// errorはない
///
pub fn none<'a>(i: &'a str) -> (&'a str, ()) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // パースされないことを確認
    #[test]
    fn not_parse_test() {
        let base = "  abc";
        let (rest, _) = none(base);
        // 残った文字列
        assert_eq!(rest, "  abc");
    }

    // パースされないことを確認
    #[test]
    fn not_parse_test2() {
        let base = "  abc";
        let (rest, _) = none(base);
        // パースされてたらNG
        assert_ne!(rest, "abc");
    }
}
