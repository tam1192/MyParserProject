use crate::error::{Error, Result};
use super::Parser;

/// Parse a number from the input string
/// 
/// # Example
///
/// ``` rust
/// use my_parser_project::parser::simple::num;
///
/// let base = "123abc";
/// let parser = num;
/// assert_eq!(parser(base), Ok(("abc", 123)));
/// ```
pub fn num<'a>(i: &'a str) -> Result<(&'a str, i64)> {
    let end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    let num = i[..end].parse::<i64>()?;
    Ok((&i[end..], num))
}

/// Parse a character from the input string
/// 
/// # Example
/// 
/// ``` rust
/// use my_parser_project::parser::simple::char;
/// 
/// let base = "/123";
/// let pattern = '/';
/// let parser = char(pattern);
/// assert_eq!(parser(base), Ok(("123", ())));
/// ```
pub fn char<'a>(c: char) -> impl Parser<&'a str, ()> 
{
    move |i: &'a str| {
        if i.starts_with(c) {
            Ok((&i[1..], ()))
        } else {
            Err(Error::ParseCharError)
        }
    }
}

pub fn space_trimer<'a, T>(parser: impl Parser<&'a str, T>) -> impl Parser<&'a str, T> {
    move |i: &'a str| {
        parser(i.trim_start())
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

#[cfg(test)]
mod num_test {
    use super::*;

    #[test]
    fn test1() {
        let base = "123abc";
        let parser = num;
        assert_eq!(parser(base), Ok(("abc", 123)));
    }

    #[test]
    fn test2() {
        let base = "abc123";
        let parser = num;
        assert!(matches!(parser(base), Err(Error::ParseIntErrror(_))));
    }

}

#[cfg(test)]
mod space_trimer_test {
    use super::*;
    #[test]
    fn test1() {
        let base = " 123";
        let parser = space_trimer(num);
        assert_eq!(parser(base), Ok(("", 123)));
    }

    #[test]
    fn test2() {
        let base = "123abc";
        let parser = space_trimer(num);
        assert_eq!(parser(base), Ok(("abc", 123)));
    }

    #[test]
    fn test3() {
        let base = "\t123abc";
        let parser = space_trimer(num);
        assert_eq!(parser(base), Ok(("abc", 123)));
    }

    #[test]
    fn test4() {
        let base = "\n123abc";
        let parser = space_trimer(num);
        assert_eq!(parser(base), Ok(("abc", 123)));
    }
}
