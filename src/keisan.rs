use crate::{error::*, parser::simple::*};

#[derive(Debug)]
pub enum OPs {
    Add,
    Neg,
    Mul,
    Div,
}

impl OPs {
    fn new<'a>(input: &'a str) -> Result<(&'a str, Self), &'a str> {
        let input = input.trim_start();
        match input.trim_ascii_start().chars().next() {
        //     Some('+') => Ok((input, Self::Add)),
        //     Some('-') => Ok((input, Self::Neg)),
        //     Some('*') => Ok((input, Self::Mul)),
        //     Some('/') => Ok((input, Self::Div)),
            _ => Err(Error::ParseCharError),
        }
    }
}



pub fn parser<'a>(input: &'a str) -> Result<(&'a str, Number), &'a str> {
    let num_parse = space_trimer(num_ex);
    let sym_parse = space_trimer(char('+'));
    
    // first value
    let (input, first) = num_parse(input)
        .map_err(|_| Error::ParseError(input))?;
    // synbol check
    let (input, _) = sym_parse(input)
        .map_err(|_| Error::ParseError(input))?;
    // second value
    let (input, second) = num_parse(input)
        .map_err(|_| Error::ParseError(input))?;

    if let Number::Int(first) = first {
        if let Number::Int(second) = second {
            return Ok((input, Number::Int(first + second)));
        }
    }
    let first = match first {
        Number::Int(x) => x as f64,
        Number::Float(x) => x,
    };
    let second = match second {
        Number::Int(x) => x as f64,
        Number::Float(x) => x,
    };
    Ok((input, Number::Float(first + second)))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let base = "1+1";
        assert_eq!(parser(base), Ok(("", Number::Int(2))));
    }

    #[test]
    fn test2() {
        let base = "2 + 4";
        assert_eq!(parser(base), Ok(("", Number::Int(6))));
    }

    #[test]
    fn test3() {
        let base = "aa1+1ffad";
        assert_eq!(parser(base), Err(Error::ParseError("aa1+1ffad")));
    }

    #[test]
    fn test4() {
        let base = "1 + a 1";
        assert_eq!(parser(base), Err(Error::ParseError(" a 1")));
    }

    #[test]
    fn test5() {
        let base = "1 + 1aaaaaa";
        assert_eq!(parser(base), Ok(("aaaaaa", Number::Int(2))));
    }

}