use crate::error::*;

enum Number {
    Int(i64),
    Float(f64),
}

fn parser<'a>(input: &'a str) -> Result<(&'a str, Number)> {
    Err(Error::Uninstalled)
}
