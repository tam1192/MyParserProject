use crate::error::Result;

pub fn trimer<'a>(input: &'a str) -> Result<(&'a str, ()), &'a str> {
    Ok((input.trim_start(), ()))
}

#[cfg(test)]
mod tests {
    use crate::parser::{num, AndParse};

    use super::*;
    #[test]
    fn test1() {
        let base = " 123";
        let parser = trimer.and(num);
        assert_eq!(parser(base), Ok(("", ((), 123))));
    }

    #[test]
    fn test2() {
        let base = "123abc";
        let parser = trimer.and(num);
        assert_eq!(parser(base), Ok(("abc", ((), 123))));
    }

    #[test]
    fn test3() {
        let base = "\t123abc";
        let parser = trimer.and(num);
        assert_eq!(parser(base), Ok(("abc", ((), 123))));
    }

    #[test]
    fn test4() {
        let base = "\n123abc";
        let parser = trimer.and(num);
        assert_eq!(parser(base), Ok(("abc", ((), 123))));
    }
}
