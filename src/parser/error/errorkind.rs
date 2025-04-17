/// エラーの種類を表すenumです。
///
/// - [Error::kind][super::Error::kind] で使用されます。
/// - 各項目の詳しい解説は、エラーを返す関数の項目を参照してください。
#[derive(PartialEq, Debug)]
// 各Kindの解説は、実際にエラーを返す関数の項で説明
pub enum ErrorKind {
    /// [num][crate::parser::str_parser::num] パーサーのエラー
    ParseNumError(std::num::ParseIntError),
    /// [char][crate::parser::str_parser::char] パーサーのエラー
    ParseCharError,
    /// [string][crate::parser::str_parser::string] パーサーのエラー
    ParseStringError,
}
