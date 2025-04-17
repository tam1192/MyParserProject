//! 単体で動作する、基本的なパーサーです。
//!
//! このモジュールは、文字列や数値をパースするための基本的なパーサーを提供します。
use super::*;

mod num;
pub use num::*;

mod char;
pub use char::*;

mod string;
pub use string::*;

mod trimer;
pub use trimer::*;

mod none;
pub use none::*;
