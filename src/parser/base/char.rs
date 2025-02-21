use crate::error::Error;
use crate::parser::Parser;

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
pub fn char<'a>(c: char) -> impl Parser<&'a str, ()> {
    move |i: &'a str| {
        if i.starts_with(c) {
            Ok((&i[1..], ()))
        } else {
            Err(Error::ParseCharError)
        }
    }
}

#[cfg(test)]
mod char_test {
    use super::*;

    #[test]
    fn test1() {
        let base = "/123";
        let pattern = '/';
        let parser = char(pattern);
        assert_eq!(parser(base), Ok(("123", ())));
    }

    #[test]
    fn test2() {
        let base = "abc/123";
        let pattern = '/';
        let parser = char(pattern);
        assert_eq!(parser(base), Err(Error::ParseCharError));
    }

    #[test]
    fn test3() {
        let base = "/123";
        let pattern = '*';
        let parser = char(pattern);
        assert_eq!(parser(base), Err(Error::ParseCharError));
    }

    #[test]
    fn test4() {
        let base = "";
        let pattern = ' ';
        let parser = char(pattern);
        assert_eq!(parser(base), Err(Error::ParseCharError));
    }
}
