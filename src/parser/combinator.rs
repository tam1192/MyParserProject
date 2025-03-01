mod and_parse;

/// Combine parser
/// It is implemented in a parser object.
/// The parsers put in the method arguments are connected.
///
/// # Error
/// Error::CombinatorParseError(Box::new(e), None)
///
/// # Example
/// ```rust
/// use my_parser_project::parser::{base::{num, char}, combinator::AndParse};
///
/// let base = "123+abc";
/// let parser = num.and(char('+'));
/// assert_eq!(parser(base), Ok(("abc", (123, ()))))
/// ```
pub use and_parse::AndParse;

mod or_parse;
/// Choose a parser
pub use or_parse::{OrParse, OrResult};

mod map_parse;
/// Apply function to parser
pub use map_parse::MapParse;
