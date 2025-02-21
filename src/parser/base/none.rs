pub fn none<'a>(i: &'a str) -> Result<(&'a str, ()), &'a str> {
    Ok((i, ()))
}

#[cfg(test)]
mod char_test {
    use super::*;

    #[test]
    fn test1() {
        let base = "/123";
        assert_eq!(none(base), Ok(("/123", ())));
    }
}
