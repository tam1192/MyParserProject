use super::error::*;
use super::parser::*;

/// Parse a number from the input string
/// example:
/// ```
/// use MyParserProject::simple_perser::base::num;
/// let base = "123abc";
/// let parser = num;
/// assert_eq!(parser(base), Ok(("abc", 123)));
/// ```
pub fn num<'a>(i: &'a str) -> Result<(&'a str, i64)> {
    let end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    let num = i[..end].parse::<i64>()?;
    Ok((&i[end..], num))
}

/// Parse a char from the input string
/// example:
/// ```
/// use MyParserProject::simple_perser::base::char;
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
