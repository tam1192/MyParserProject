/// パースしない
///
/// パースをしない [Parser][super::Parser] です。
///
/// # 用途について
/// [Substitute][super::combinator::Substitute]で、"A" or "Other"を表現するために使われます。  
/// 詳しくはExampleの項を参考にしてください。  
///
/// # Example
/// ```rust
/// use my_parser_project::parser::{str_parser::{num, none}, combinator::*};
///
/// // 数値があれば、数値を取り出す
/// let parser = num.sub_uncheck(none);
///
/// // 数値なし
/// let case1 = "abc";
/// let (rest, result) = parser(case1);
/// assert_eq!(result, SubResult::Sub(()));
/// assert_eq!(rest, "abc");
///
/// // 数値あり
/// let case2 = "123abc";
/// let (rest, result) = parser(case2);
/// assert_eq!(result, SubResult::Main(123));
/// assert_eq!(rest, "abc");
/// ```
pub fn none<'a>(i: &'a str) -> (&'a str, ()) {
    (i, ())
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
