use crate::{error::*, number::Number, parser::*};

// 電卓メモ
// <Expression> ::= <Term> | <Expression> '+' <Term> | <Expression> '-' <Term>
// <Term> ::= <Exponent> | <Term> '*' <Exponent> | <Term> '/' <Exponent>
// <Exponent> ::= <Factor> | <Exponent> '^' <Factor>
// <Factor> ::= Number | '(' <Expression> ')'

#[derive(Debug,  PartialEq)]
pub enum Factor {
    Number(Number),
    Scope(Box<Expression>),
}

impl Factor {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
        trimer.and_b(
            num_ex.map(|n| Self::Number(n)).or(char('(')
                .and_b(trimer)
                .and_b(Expression::new)
                .and_a(trimer)
                .and_a(char(')'))
                .map(|e| Self::Scope(Box::new(e)))),
        )(i)
    }

    fn calc(&self) -> Number {
        match self {
            Factor::Number(number) => number.clone(),
            Factor::Scope(expression) => expression.calc(),
        }
    }
}

#[derive(Debug,  PartialEq)]
pub enum Exponent {
    Factor(Box<Factor>),
    Power(Box<Exponent>, Factor),
}

impl Exponent {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
        trimer.and_b(
            Factor::new.map(|f| Self::Factor(Box::new(f))).or(Self::new
                .and(trimer.and_b(char('^')).and_b(trimer).and_b(Factor::new))
                .map(|(e, f)| Self::Power(Box::new(e), f))),
        )(i)
    }
    fn calc(&self) -> Number {
        match self {
            Exponent::Factor(factor) => factor.calc(),
            Exponent::Power(exponent, factor) => todo!(),
        }
    }
}

#[derive(Debug,  PartialEq)]
pub enum Term {
    Exponent(Box<Exponent>),
    Mul(Box<Term>, Exponent),
    Div(Box<Term>, Exponent),
}

impl Term {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
        trimer.and_b(
            Exponent::new
                .map(|e| Self::Exponent(Box::new(e)))
                .or(Self::new
                    .and(trimer.and_b(char('*')).and_b(trimer).and_b(Exponent::new))
                    .map(|(t, e)| Self::Mul(Box::new(t), e)))
                .or(Self::new
                    .and(trimer.and_b(char('/')).and_b(trimer).and_b(Exponent::new))
                    .map(|(t, e)| Self::Div(Box::new(t), e))),
        )(i)
    }

    fn calc(&self) -> Number {
        match self {
            Term::Exponent(exponent) => todo!(),
            Term::Mul(term, exponent) => todo!(),
            Term::Div(term, exponent) => todo!(),
        }
    }
}

#[derive(Debug,  PartialEq)]
pub enum Expression {
    Term(Box<Term>),
    Add(Box<Expression>, Term),
    Sub(Box<Expression>, Term),
}

impl Expression {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
        trimer.and_b(
            Term::new
                .map(|t| Self::Term(Box::new(t)))
                .or(Self::new
                    .and(trimer.and_b(char('+')).and_b(trimer).and_b(Term::new))
                    .map(|(t, e)| Self::Add(Box::new(t), e)))
                .or(Self::new
                    .and(trimer.and_b(char('-')).and_b(trimer).and_b(Term::new))
                    .map(|(t, e)| Self::Sub(Box::new(t), e))),
        )(i)
    }
    fn calc(&self) -> Number {
        match self {
            Expression::Term(term) => todo!(),
            Expression::Add(expression, term) => todo!(),
            Expression::Sub(expression, term) => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum OPs {
    Add(Number),
    Sub(Number),
    Mul(Number),
    Div(Number),
}

impl OPs {
    pub fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
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

pub fn parser<'a>(i: &'a str) -> Result<(&'a str, Number)> {
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

    #[test]
    fn test2_1() {
        let base = "10";
        let parse = Expression::new(base);
        // let ans = Expression::Term(Term::Exponent(Exponent::Factor(Factor::Number(10))));
        // assert_eq!(parse, Ok(("", Factor::Number(Number::Int(10)))))
    }

}
