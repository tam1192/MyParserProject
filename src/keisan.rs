use crate::{error::*, parser::simple::*};


fn parser<'a>(input: &'a str) -> Result<(&'a str, Number)> {
    let num_parse = space_trimer(num_ex);
    let sym_parse = space_trimer(char('+'));
    
    let (input, first) = num_parse(input)?;
    let (input, _) = sym_parse(input)?;
    println!("{}", input);
    let (input, second) = num_parse(input)?;

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
        assert_eq!(parser(base), Err(Error::ParseError("aa1+1ffad".to_string())));
    }

    #[test]
    fn test4() {
        let base = "1 + a 1";
        assert_eq!(parser(base), Err(Error::ParseError("1 + a 1".to_string())));
    }

    #[test]
    fn test5() {
        let base = "1 + 1aaaaaa";
        assert_eq!(parser(base), Ok(("aaaaaa", Number::Int(2))));
    }
}