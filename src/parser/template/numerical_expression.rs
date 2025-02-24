use crate::{error::*, number::Number, parser::*};

// 電卓メモ
// ↓例えばExpression, TermとExpression逆でよくね？
// <Expression> ::= <Term> | <Expression> '+' <Term> | <Expression> '-' <Term>
// <Term> ::= <Exponent> | <Term> '*' <Exponent> | <Term> '/' <Exponent>
// <Exponent> ::= <Factor> | <Exponent> '^' <Factor>
// <Factor> ::= Number | '(' <Expression> ')'

#[derive(Debug, PartialEq)]
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

    fn calc(&self) -> Result<Number> {
        Ok(match self {
            Factor::Number(number) => number.clone(),
            Factor::Scope(expression) => expression.calc()?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Exponent {
    Factor(Factor),
    Power(Factor, Box<Self>),
}

impl Exponent {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
        Factor::new
            .and(
                trimer
                    .and_b(char('^'))
                    .and_b(trimer)
                    .and_b(Self::new)
                    .or_ab(none),
            )
            .map(|(f, o)| match o {
                OrResult::A(e) => Self::Power(f, Box::new(e)),
                OrResult::B(_) => Self::Factor(f),
            })(i)
    }
    fn calc(&self) -> Result<Number> {
        Ok(match self {
            Exponent::Factor(factor) => factor.calc()?,
            Exponent::Power(exponent, factor) => {
                let x = exponent.calc()?;
                let y = factor.calc()?;
                x.pow(y)?
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Exponent(Exponent),
    Mul(Exponent, Box<Self>),
    Div(Exponent, Box<Self>),
}

impl Term {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
        trimer
            .and_b(
                Exponent::new.and(
                    trimer
                        .and_b(char('*').and_b(trimer).and_b(Self::new))
                        .or_ab(
                            trimer
                                .and_b(char('/').and_b(trimer).and_b(Self::new))
                                .or_ab(none),
                        ),
                ),
            )
            .map(|(e, o)| match o {
                OrResult::A(t) => Self::Mul(e, Box::new(t)),
                OrResult::B(o) => match o {
                    OrResult::A(t) => Self::Div(e, Box::new(t)),
                    OrResult::B(_) => Self::Exponent(e),
                },
            })(i)
    }

    fn calc(&self) -> Result<Number> {
        Ok(match self {
            Term::Exponent(exponent) => exponent.calc()?,
            Term::Mul(term, exponent) => {
                let x = term.calc()?;
                let y = exponent.calc()?;
                x * y
            }
            Term::Div(term, exponent) => {
                let x = term.calc()?;
                let y = exponent.calc()?;
                (x / y)?
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Term(Term),
    Add(Term, Box<Self>),
    Sub(Term, Box<Self>),
}

impl Expression {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self)> {
        trimer
            .and_b(
                Term::new.and(
                    trimer
                        .and_b(char('+').and_b(trimer).and_b(Self::new))
                        .or_ab(
                            trimer
                                .and_b(char('-').and_b(trimer).and_b(Self::new))
                                .or_ab(none),
                        ),
                ),
            )
            .map(|(e, o)| match o {
                OrResult::A(t) => Self::Add(e, Box::new(t)),
                OrResult::B(o) => match o {
                    OrResult::A(t) => Self::Sub(e, Box::new(t)),
                    OrResult::B(_) => Self::Term(e),
                },
            })(i)
    }
    fn calc(&self) -> Result<Number> {
        Ok(match self {
            Expression::Term(term) => term.calc()?,
            Expression::Add(expression, term) => {
                let x = expression.calc()?;
                let y = term.calc()?;
                x + y
            }
            Expression::Sub(expression, term) => {
                let x = expression.calc()?;
                let y = term.calc()?;
                x - y
            }
        })
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
            .and_b(trimer.and_b(num_ex))
            .map(|n| Self::Add(n))
            .or(char('-').and_b(trimer.and_b(num_ex)).map(|n| Self::Sub(n)))
            .or(char('*').and_b(trimer.and_b(num_ex)).map(|n| Self::Mul(n)))
            .or(char('/').and_b(trimer.and_b(num_ex)).map(|n| Self::Div(n)))(i)
    }

    fn calc(&self, x: Number) -> Number {
        match self {
            OPs::Add(y) => x + *y,
            OPs::Sub(y) => x - *y,
            OPs::Mul(y) => x * *y,
            OPs::Div(y) => (x / *y).unwrap(),
        }
    }
}

pub fn parser<'a>(i: &'a str) -> Result<(&'a str, Number)> {
    let (i, n) = trimer.and_b(num_ex)(i)?;
    let p = trimer.and_b(OPs::new).map(|o| o.calc(n));
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
            ("2.0 + 2.0add", ("add", Number::Float(4.0))),
            ("2.0 - 2.0sub", ("sub", Number::Float(0.0))),
            ("2.0 * 2.0mul", ("mul", Number::Float(4.0))),
            ("2.0 / 2.0div", ("div", Number::Float(1.0))),
        ];

        for (base, ans) in base {
            let (s, o) = Expression::new(base).unwrap();
            let n = o.calc().unwrap();
            assert_eq!((s, n), ans)
        }
    }

    #[test]
    fn test2_1() {
        let base = "10 + 2";
        let (_, o) = Expression::new(base).unwrap();
        let a = o.calc().unwrap();
        println!("{:?}", a);
    }
}
