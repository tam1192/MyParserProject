use crate::parser::Parser;

pub fn trimer<'a, O>(parser: impl Parser<&'a str, O>) -> impl Parser<&'a str, O> {
    move |i| {
        parser(i.trim_start())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::num;

    use super::*;
    #[test]
    fn test1() {
        let base = " 123";
        let parser = trimer(num);
        assert_eq!(parser(base), Ok(("", 123)));
    }

    #[test]
    fn test2() {
        let base = "123abc";
        let parser = trimer(num);
        assert_eq!(parser(base), Ok(("abc", 123)));
    }

    #[test]
    fn test3() {
        let base = "\t123abc";
        let parser = trimer(num);
        assert_eq!(parser(base), Ok(("abc", 123)));
    }

    #[test]
    fn test4() {
        let base = "\n123abc";
        let parser = trimer(num);
        assert_eq!(parser(base), Ok(("abc", 123)));
    }
}
