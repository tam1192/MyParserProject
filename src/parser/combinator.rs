mod and_parse;

/// # Combine parser
///
/// It is implemented in a parser object.  
/// The parsers put in the method arguments are connected.  
///
/// ## What are A and B?
/// - The object side (self) is A
/// - The side included in the method argument is B
///
/// ## Error
/// [super::Error::AndParseError]  
/// If is_b is true, error occurred on B side
///
/// ## Example
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
///
/// ## What are A and B?
/// - The object side (self) is A
/// - The side included in the method argument is B
///
/// ## Error
/// [super::Error::OrParseError]  
/// The [std::error::Error::source()] method cannot be used because two sources are included.  
/// It is necessary to use the [super::Error::division_combinator_parse_error] method to split them.  
///
/// ## Example
/// ```rust
/// use my_parser_project::parser::{base::{num, char}, combinator::{OrParse, OrResult}};
///
/// let base = "123+abc";
/// let parser = num.or_ab(char('+'));
/// assert_eq!(parser(base), Ok(("+abc", OrResult::A(123))))
/// ```
///
pub use or_parse::{OrParse, OrResult};

mod map_parse;
/// Apply function to parser
pub use map_parse::MapParse;
