/// 空白を除去する
///
/// 空白を条件に解析する [Parser][super::Parser] です。
///
/// - 先頭から解析を行います。
///
/// # Example
/// ```rust
/// use crate::parser::base::trimer;
///
/// let input = "     abc";
/// let (rest, _) = trimer(input);
/// assert_eq!(rest, "abc");
/// ```
pub fn trimer<'a>(i: &'a str) -> (&'a str, ()) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // スペース
    #[test]
    fn space_trim_test() {
        let base = "  abc";
        let (rest, _) = trimer(base);
        // 残った文字列
        assert_eq!(rest, "abc");
    }

    // タブ
    #[test]
    fn tab_trim_test() {
        let base = "\tabc";
        let (rest, _) = trimer(base);
        // 残った文字列
        assert_eq!(rest, "abc");
    }

    // 改行
    #[test]
    fn return_trim_test() {
        let base = "\nabc";
        let (rest, _) = trimer(base);
        // 残った文字列
        assert_eq!(rest, "abc");
    }
}
