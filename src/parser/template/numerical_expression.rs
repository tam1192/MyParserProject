use crate::{error::*, number::Number, parser::*};

#[derive(Debug)]
pub enum OPs {
    Add(Number),
    Sub(Number),
    Mul(Number),
    Div(Number),
}

impl OPs {
    pub fn new<'a>(i: &'a str) -> Result<(&'a str, Self), &'a str> {
        char('+')
            .and(trimer.and(num_ex))
            .map(|(_, (_, x))| Self::Add(x))
            .or(char('-')
                .and(trimer.and(num_ex))
                .map(|(_, (_, x))| Self::Sub(x)))
            .or(char('*')
                .and(trimer.and(num_ex))
                .map(|(_, (_, x))| Self::Mul(x)))
            .or(char('/')
                .and(trimer.and(num_ex))
                .map(|(_, (_, x))| Self::Div(x)))(i)
    }

    fn calc(&self, x: Number) -> Number {
        match self {
            OPs::Add(y) => x + *y,
            OPs::Sub(y) => x - *y,
            OPs::Mul(y) => x * *y,
            OPs::Div(y) => x / *y,
        }
    }
}

pub fn parser<'a>(i: &'a str) -> Result<(&'a str, Number), &'a str> {
    let (i, (_, n)) = trimer.and(num_ex)(i)?;
    let p = trimer.and(OPs::new).map(|(_, o)| o.calc(n));
    p(i)
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
        let base = vec![
            ("2.0 + 2.0add", ("add", Number::Float(2.0 + 2.0))),
            ("2.0 - 2.0sub", ("sub", Number::Float(2.0 - 2.0))),
            ("2.0 * 2.0mul", ("mul", Number::Float(2.0 * 2.0))),
            ("2.0 / 2.0div", ("div", Number::Float(2.0 / 2.0))),
        ];

        for (base, ans) in base {
            assert_eq!(parser(base), Ok(ans))
        }
    }
}
