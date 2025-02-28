use crate::number::Number;
use crate::parser::error::Error;

pub fn num_ex<'a>(i: &'a str) -> Result<(&'a str, Number), Error> {
    // 整数部を確認
    let integer_end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    if integer_end != i.len() {
        // '.'を確認
        if i[integer_end..].chars().next() == Some('.') {
            let dot_end = integer_end + 1;
            // '.'がある場合は小数として処理
            let float_end = i[dot_end..]
                .find(|c: char| !c.is_numeric())
                .unwrap_or(i[dot_end..].len());
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

#[cfg(test)]
mod tests {
    use super::num_ex as parser;
    use crate::number::Number;
    use crate::parser::error::Error;

    #[test]
    fn test1() {
        let base = "123abc";
        assert_eq!(parser(base), Ok(("abc", Number::Int(123))));
    }

    #[test]
    fn test2() {
        let base = "abc123";
        assert!(matches!(parser(base), Err(Error::ParseIntErrror(_))));
    }

    #[test]
    fn test3() {
        let base = "123.abc";
        assert_eq!(parser(base), Ok(("abc", Number::Float(123.0))));
    }

    #[test]
    fn test4() {
        let base = "123.14abc";
        assert_eq!(parser(base), Ok(("abc", Number::Float(123.14))));
    }

    #[test]
    fn test5() {
        let base = "123";
        assert_eq!(parser(base), Ok(("", Number::Int(123))));
    }

    #[test]
    fn test6() {
        let base = "123.14";
        assert_eq!(parser(base), Ok(("", Number::Float(123.14))));
    }
}
