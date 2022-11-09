use std::fmt;

#[derive(Debug)]
pub enum Error {
    NoRegexMatches,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoRegexMatches => f.write_str("NoRegexMatches"),
        }
    }
}
