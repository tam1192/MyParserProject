use crate::parser::error::Error;

pub fn num<'a>(i: &'a str) -> Result<(&'a str, i64), Error> {
    let end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    let num = i[..end].parse::<i64>()?;
    Ok((&i[end..], num))
}

#[cfg(test)]
mod tests {
    use super::num as parser;
    use crate::parser::error::Error;

    #[test]
    fn test1() {
        let base = "123abc";
        assert_eq!(parser(base), Ok(("abc", 123)));
    }

    #[test]
    fn test2() {
        let base = "abc123";
        assert!(matches!(parser(base), Err(Error::ParseIntErrror(_))));
    }

    #[test]
    fn test3() {
        let base = "123.abc";
        assert_eq!(parser(base), Ok((".abc", 123)));
    }
}
