use crate::error::*;

#[derive(Debug, PartialEq)]
enum Number {
    Int(i64),
    Float(f64),
}

fn parser<'a>(input: &'a str) -> Result<(&'a str, Number)> {
    Err(Error::Uninstalled)
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