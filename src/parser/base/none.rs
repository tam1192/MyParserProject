use super::*;

/// パースしない
///
/// パースをしない [super::Parser] を返します
///
/// # 用途について
/// [super::combinator::Substitute]で、"A" or "Other"を表現するために使われます。
/// 詳しくはExampleの項を参考にしてください。
///
/// # Example
/// ```rust
/// use crate::parser::base::{num, none};
///
/// // 数値があれば、数値を取り出す
/// let parser = num.sub(none);
///
/// // 数値なし
/// let case1 = "abc";
///
///
/// // 数値あり
/// let case2 = "123abc";
/// let (rest)
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
