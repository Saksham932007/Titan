use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidRequest => write!(f, "Invalid HTTP request"),
            ParseError::InvalidEncoding => write!(f, "Invalid UTF-8 encoding"),
            ParseError::InvalidProtocol => write!(f, "Invalid HTTP protocol"),
            ParseError::InvalidMethod => write!(f, "Invalid HTTP method"),
        }
    }
}

impl Error for ParseError {}
