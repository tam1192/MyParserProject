use crate::error::Result;

/// Parse a number from the input string
/// 
/// # Example
///
/// ``` rust
/// use my_parser_project::parser::num;
///
/// let base = "123abc";
/// let parser = num;
/// assert_eq!(parser(base), Ok(("abc", 123)));
/// ```
pub fn num<'a>(i: &'a str) -> Result<(&'a str, i64), &'a str> {
    let end = i.find(|c: char| !c.is_numeric()).unwrap_or(i.len());
    let num = i[..end].parse::<i64>()?;
    Ok((&i[end..], num))
}





#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::*;

    #[test]
    fn test1() {
        let base = "123abc";
        let parser = num;
        assert_eq!(parser(base), Ok(("abc", 123)));
    }

    #[test]
    fn test2() {
        let base = "abc123";
        let parser = num;
        assert!(matches!(parser(base), Err(Error::ParseIntErrror(_))));
    }

    #[test]
    fn test3() {
        let base = "123.abc";
        let parser = num;
        assert_eq!(parser(base), Ok((".abc", 123)));
    }

}

