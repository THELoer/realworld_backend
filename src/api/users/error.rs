use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    UserDidNotExists(String),
    PasswordOrLoginIsIncorrect(String),
    ServerError(String),
    DatabaseError(String),
    DOESNOTIMPLEDERROR(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ServerError(s) => write!(f, "{}", s),
            Error::PasswordOrLoginIsIncorrect(s) => write!(f, "{}", s),
            Error::DatabaseError(s) => write!(f, "{}", s),
            Error::UserDidNotExists(s) => write!(f, "{}", s),
            Error::DOESNOTIMPLEDERROR(s) => write!(f, "{}", s),
        }
    }
}





impl std::error::Error for Error {}