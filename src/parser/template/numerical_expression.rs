use crate::{
    number::{self, Number},
    parser::{self, char, none, num_ex, trimer, AndParse, MapParse, OrParse, OrResult},
};

/// Extracting mathematical expressions from a string
///
/// # example
/// ## General usage
/// ```rust
/// use my_parser_project::parser::numerical_expression::*;
/// use my_parser_project::number::Number;
///
/// let base = "10 + 2";
/// let (_, o) = parser(base).unwrap();
/// let a = o.calc().unwrap();
/// assert_eq!(a, Number::Int(12))
/// ```
///
/// ## About Data Structure
/// ```rust
/// use my_parser_project::parser::numerical_expression::*;
/// use my_parser_project::number::Number;
///
/// let base = "2+3*4";
/// let (_, e) = parser(base).unwrap();
/// let ans =
/// Expression::Add(
///     Term::Exponent(
///         Exponent::Factor(
///             Factor::Number(
///                 Number::Int(2)
///             )
///         )
///     ),
///     Box::new(Expression::Term(
///         Term::Mul(
///             Exponent::Factor(
///                 Factor::Number(
///                     Number::Int(3)
///                 )
///             ),
///             Box::new(Term::Exponent(
///                 Exponent::Factor(
///                     Factor::Number(
///                         Number::Int(4)
///                     )
///                 )
///             ))
///         ))
///     )
/// );
/// assert_eq!(e, ans);
/// let n = e.calc().unwrap();
/// assert_eq!(n, Number::Int(14))
/// ```
///

pub fn parser<'a>(i: &'a str) -> Result<(&'a str, Expression), parser::error::Error> {
    Expression::new(i)
}

/// Executes `parser()` and also performs calculations
///
/// # example
/// ```rust
/// use my_parser_project::parser::numerical_expression::*;
/// use my_parser_project::number::Number;
///
/// let base = "10 + 2";
/// assert_eq!(parse_and_calc(base).unwrap(), ("", Number::Int(12)))
/// ```
pub fn parse_and_calc<'a>(i: &'a str) -> Result<(&'a str, Number), parser::error::Error> {
    let (i, e) = parser(i)?;
    let a = e.calc()?;
    Ok((i, a))
}

#[derive(Debug, PartialEq)]
pub enum Factor {
    Number(Number),
    Scope(Box<Expression>),
}

impl Factor {
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self), parser::error::Error> {
        trimer.and_b(
            num_ex.map(|n| Self::Number(n)).or(char('(')
                .and_b(trimer)
                .and_b(Expression::new)
                .and_a(trimer)
                .and_a(char(')'))
                .map(|e| Self::Scope(Box::new(e)))),
        )(i)
    }

    fn calc(&self) -> Result<Number, number::error::Error> {
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
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self), parser::error::Error> {
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
    fn calc(&self) -> Result<Number, number::error::Error> {
        Ok(match self {
            Exponent::Factor(factor) => factor.calc()?,
            Exponent::Power(exponent, factor) => {
                let x = exponent.calc()?;
                let y = factor.calc()?;
                x.pow(y)
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
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self), parser::error::Error> {
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

    fn calc(&self) -> Result<Number, number::error::Error> {
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
    fn new<'a>(i: &'a str) -> Result<(&'a str, Self), parser::error::Error> {
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
    pub fn calc(&self) -> Result<Number, number::error::Error> {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let base = "1+1";
        assert_eq!(parse_and_calc(base), Ok(("", Number::Int(2))));
    }

    #[test]
    fn test2() {
        let base = "2 + 4";
        assert_eq!(parse_and_calc(base), Ok(("", Number::Int(6))));
    }

    #[test]
    fn test3() {
        let base = "aa1+1ffad";
        assert!(matches!(parse_and_calc(base), Err(_)));
    }

    #[test]
    fn test4() {
        let base = "1 + a 1";
        assert_eq!(parse_and_calc(base), Ok((" + a 1", Number::Int(1))))
    }

    #[test]
    fn test5() {
        let base = "1 + 1aaaaaa";
        assert_eq!(parse_and_calc(base), Ok(("aaaaaa", Number::Int(2))));
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
        println!("{:?}", o);
        let a = o.calc().unwrap();
        println!("{:?}", a);
    }

    #[rustfmt::skip]
    #[test]
    fn test2_2() {
        let base = "2+3*4";
        let (_, e) = parser(base).unwrap();
        // 中身
        let ans = 
        Expression::Add(
            Term::Exponent(
                Exponent::Factor(
                    Factor::Number(
                        Number::Int(2)
                    )
                )
            ), 
            Box::new(Expression::Term(
                Term::Mul(
                    Exponent::Factor(
                        Factor::Number(
                            Number::Int(3)
                        )
                    ),
                    Box::new(Term::Exponent(
                        Exponent::Factor(
                            Factor::Number(
                                Number::Int(4)
                            )
                        )
                    )) 
                ))
            )
        );
        assert_eq!(e, ans);
        let n = e.calc().unwrap();
        assert_eq!(n, Number::Int(14))
    }

    #[test]
    fn test2_3() {
        let base = "(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9)*2";
        let (_, n) = parse_and_calc(base).unwrap();
        assert_eq!(n, Number::Int(90))
    }
}
