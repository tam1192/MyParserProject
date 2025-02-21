/// Parse a character from the input string
///
/// # Example
///
/// ``` rust
/// use my_parser_project::parser::char;
///
/// let base = "/123";
/// let pattern = '/';
/// let parser = char(pattern);
/// assert_eq!(parser(base), Ok(("123", ())));
/// ```
mod char;
pub use char::char;

/// Parse a number from the input string
///
/// # Example
///
/// ``` rust
/// use my_parser_project::parser::num;
///
/// let base = "123abc";
/// let parser = num;
mod num;
pub use num::num;

mod num_ex;
pub use num_ex::num_ex;

mod trimer;
pub use trimer::trimer;
