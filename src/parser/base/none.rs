use crate::parser::error::Result;

pub fn none<'a>(i: &'a str) -> Result<(&'a str, ())> {
    Ok((i, ()))
}

#[cfg(test)]
mod char_test {
    use crate::parser::{char, num, AndParse, OrParse};

    use super::*;

    #[test]
    fn test1() {
        let base = "/123";
        assert_eq!(none(base), Ok(("/123", ())));
    }

    #[test]
    fn test2() {
        let base1 = "+123";
        let base2 = "123";
        let parser = char('+').or(none).and_b(num);
        assert_eq!(parser(base1), Ok(("", 123)));
        assert_eq!(parser(base2), Ok(("", 123)));
    }
}
