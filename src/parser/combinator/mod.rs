//! 複数の[Parser]を組み合わせられる機能を提供するモジュール
//!
//! このモジュールを使用すると、既存の[Parser]に追加の解析条件を組み合わせることで、より高度な解析を行うことができます。  
//! 異なる[Parser]を統合し、新たな[Parser]を構築することが可能です。
use super::*;

mod concat;
pub use concat::*;

mod substitute;
pub use substitute::*;

mod map;
pub use map::*;

mod not;
pub use not::*;
