mod number;
pub use number::Number;

mod error;
pub use error::{Error, ErrorKind};

#[cfg(test)]
mod tests;
