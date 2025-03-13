/// [super::Error] で使用する、ジャンル分けされたエラー原因を格納するenum
#[derive(PartialEq, Debug)]
pub enum ErrorKind {
    /// パースできない時
    ParseNumError(std::num::ParseIntError),
    ParseCharError,
    ParseStringError,
}
