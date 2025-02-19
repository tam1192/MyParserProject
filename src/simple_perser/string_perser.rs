use std::{error, fmt};

#[derive(Debug)]
enum Error {
    Test(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Test(s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None 
    }
}

enum Result<T> {
    Ok(T),
    Err(Error),
}
