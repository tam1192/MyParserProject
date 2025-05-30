//! 文字列を使う基本的なパーサーを提供するモジュール
//!
//! このモジュールは、文字列や数値をパースするための基本的なパーサーを提供します。  
//! 引数は[文字列スライス][str]となり、結果は各パーサーによって異なります。  
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
