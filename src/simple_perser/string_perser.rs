use std::{error, fmt};

#[derive(Debug, PartialEq)]
enum Error {
    ParseIntErrror(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    ParseCharError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloatError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseIntErrror(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseCharError => write!(f, "ParseCharError"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ParseFloatError(e) => Some(e),
            Self::ParseIntErrror(e) => Some(e),
            Self::ParseCharError => None,
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseIntErrror(e)
    }
}   

type Result<T> = std::result::Result<T, Error>;

trait Parser<I, O>: Fn(I) -> Result<(I, O)> {}
impl <F, I, O> Parser<I, O> for F where F: Fn(I) -> Result<(I, O)> {}

fn num<'a>(i: &'a str) -> Result<(&'a str, i64)> {
    let end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    let num = i[..end].parse::<i64>()?;
    Ok((&i[end..], num))
}


fn char<'a>(c: char) -> impl Parser<&'a str, ()> 
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
