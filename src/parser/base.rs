mod char;

/// Parse a character from the input string.  
///
/// # Error
/// if not found, return error type [super::Error::ParseCharError].
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
/// The numeric (0-9) characters are obtained from the beginning until they are broken off,  
/// Then [`str::parse::<i64>`()] converts it.
///
/// # Error
/// [`str::parse::<i64>`()] fails [super::Error::ParseIntError] type error returns
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
///
/// # Should I use num or num_ex?
/// If decimal point is not needed, num is recommended
///
/// # Error
/// - [`str::parse::<i64>`()] fails [super::Error::ParseIntError] type error returns
/// - [`str::parse::<f64>`()] fails [super::Error::ParseFloatError] type error returns
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

mod trimer;
/// space trim
///
/// # Error
/// No errors exist at all.
pub use trimer::trimer;

mod none;
/// None parser
///
/// # Error
/// No errors exist at all.
pub use none::none;
