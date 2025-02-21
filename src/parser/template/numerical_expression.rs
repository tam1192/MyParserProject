use crate::{error::*, number::Number, parser::*};

#[derive(Debug)]
pub enum OPs {
    Add,
    Sub,
    Mul,
    Div,
}

impl OPs {
    pub fn new<'a>(i: &'a str) -> Result<(&'a str, Self), &'a str> {
        trimer
            .and(
                char('+')
                    .map(|_| Self::Add)
                    .or(char('-').map(|_| Self::Sub))
                    .or(char('*').map(|_| Self::Mul))
                    .or(char('/').map(|_| Self::Sub)),
            )
            .map(|((), x)| x)(i)
    }

    fn calc(&self, x: Number, y: Number) -> Number {
        match self {
            OPs::Add => x + y,
            OPs::Sub => x - y,
            OPs::Mul => x * y,
            OPs::Div => x / y,
        }
    }
}

pub fn parser<'a>(input: &'a str) -> Result<(&'a str, Number), &'a str> {
    trimer
        .and(num_ex)
        .and(trimer)
        .and(OPs::new)
        .and(trimer)
        .and(num_ex)
        .map(|(((((_,x), ()),ops), _),y)| {
            ops.calc(x, y)
        })(input)
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
        assert!(matches!(parser(base), Err(_)));
    }

    #[test]
    fn test4() {
        let base = "1 + a 1";
        assert!(matches!(parser(base), Err(_)));
    }

    #[test]
    fn test5() {
        let base = "1 + 1aaaaaa";
        assert_eq!(parser(base), Ok(("aaaaaa", Number::Int(2))));
    }

    #[test]
    fn test6() {
        let base = "1.0 + 4.0";
        assert_eq!(parser(base), Ok(("", Number::Float(3.0))))
    }
}
