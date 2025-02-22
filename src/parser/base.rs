mod char;

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
pub use char::char;

mod num;
/// Parse a number from the input string
///
/// # Example
///
/// ``` rust
/// use my_parser_project::parser::num;
///
/// let base = "123abc";
/// assert_eq!(num(base), Ok(("abc", 123)))
/// ``````
pub use num::num;

mod num_ex;
/// Support float for num
/// If the output can be output as an integer, it is output as an integer.
///
/// # Should I use num or num_ex?
/// If decimal point is not needed, num is recommended
///
/// # Example
///
/// ``` rust
/// use my_parser_project::parser::num_ex;
/// use my_parser_project::number::Number;
///
/// let base = "3.14abc";
/// assert_eq!(num_ex(base), Ok(("abc", Number::Float(3.14))))
/// ```
///
pub use num_ex::num_ex;

mod trim_and;
/// space trim
pub use trim_and::trim_and;

mod none;
/// None parser
pub use none::none;
