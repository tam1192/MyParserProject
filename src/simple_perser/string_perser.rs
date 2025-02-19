use std::{error, fmt};

#[derive(Debug, PartialEq)]
enum Error {
    Test(String),
    ParseIntErrror(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    DEBUG,
    ParseCharError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Test(s) => write!(f, "{}", s),
            Self::DEBUG => write!(f, "DEBUG"),
            Self::ParseFloatError(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseIntErrror(parse_int_error) => write!(f, "{}", parse_int_error),
            Self::ParseCharError => write!(f, "ParseCharError"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Test(_) => None,
            Self::DEBUG => None,
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

#[derive(Debug, PartialEq)]
enum Num {
    Int(i64),
    Float(f64),
}
fn num(i: &str) -> Result<(&str, Num)> {
    let mut end = 0;
    let mut is_float = false;
    if !i.starts_with('.') {
        for i in i.chars() {
            if i.is_numeric() {
                end += 1;
            } else if i == '.' {
                end += 1;
                is_float = true;
            } else {
                break;
            }
        }
    }
    if is_float {
        let num = i[..end].parse::<f64>()?;
        Ok((&i[end..], Num::Float(num)))
    } else {
        let num = i[..end].parse::<i64>()?;
        Ok((&i[end..], Num::Int(num)))
    }
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
        assert_eq!(num(base), Ok(("abc", Num::Int(123))));
    }

    #[test]
    fn test2() {
        let base = "abc123";
        assert!(matches!(num(base), Err(_)));
    }

    #[test]
    fn test3() {
        let base = "3.14abc";
        assert_eq!(num(base), Ok(("abc", Num::Float(3.14))));
    }

    #[test]
    fn test4() {
        let base = "3.14.abc";
        assert_eq!(num(base), Ok((".abc", Num::Float(3.14))));
    }

}
