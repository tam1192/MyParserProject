use crate::error::{Error, Result};
use super::Parser;

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

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
pub fn num<'a>(i: &'a str) -> Result<(&'a str, i64), &'a str> {
    let end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    let num = i[..end].parse::<i64>()?;
    Ok((&i[end..], num))
}

pub fn num_ex<'a>(i: &'a str) -> Result<(&'a str, Number), &'a str> {
    // 整数部を確認
    let integer_end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    if integer_end != i.len() {
        // '.'を確認
        if i[integer_end..].chars().next() == Some('.') {
            let dot_end = integer_end + 1;
            // '.'がある場合は小数として処理
            let float_end = i[dot_end..].find(|c: char | !c.is_numeric()).unwrap_or(i[dot_end..].len());
            let end = integer_end + float_end + 1;
            if let Ok(num) = i[..end].parse::<f64>() {
                return Ok((&i[end..], Number::Float(num)));
            }
        }
    }
    // '.'がない場合は整数として処理
    let num = i[..integer_end].parse::<i64>()?;
    Ok((&i[integer_end..], Number::Int(num)))
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

    #[test]
    fn test3() {
        let base = "123.abc";
        let parser = num;
        assert_eq!(parser(base), Ok((".abc", 123)));
    }


}

#[cfg(test)]
mod num_ex_test {
    use super::*;

    #[test]
    fn test1() {
        let base = "123abc";
        let parser = num_ex;
        assert_eq!(parser(base), Ok(("abc", Number::Int(123))));
    }

    #[test]
    fn test2() {
        let base = "abc123";
        let parser = num_ex;
        assert!(matches!(parser(base), Err(Error::ParseIntErrror(_))));
    }

    #[test]
    fn test3() {
        let base = "123.abc";
        let parser = num_ex;
        assert_eq!(parser(base), Ok(("abc", Number::Float(123.0))));
    }

    #[test]
    fn test4() {
        let base = "123.14abc";
        let parser = num_ex;
        assert_eq!(parser(base), Ok(("abc", Number::Float(123.14))));
    }

    #[test]
    fn test5() {
        let base = "123";
        let parser = num_ex;
        assert_eq!(parser(base), Ok(("", Number::Int(123))));
    }

    #[test]
    fn test6() {
        let base = "123.14";
        let parser = num_ex;
        assert_eq!(parser(base), Ok(("", Number::Float(123.14))));
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
