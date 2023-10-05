use super::method::RequestMethod;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: RequestMethod,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

// Buffer conversion
impl TryFrom<&[u8]> for Request {
    // Implement custom error enum
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1 -> HTTP Request to be parsed
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        unimplemented!();
    }
}


// Method to iterate over string
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

// Create an enum that contains the different errors that might be generated
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// Implement 'Display' and 'Debug' traits
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Implement From<Utf8Error> trait
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}


// Create implementation block for ParseError
impl ParseError {
    fn message(&self) -> &str {
        // Since this is the "last" expression in the function body, the string we match on will be automatically returned
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

// Make the error more idiomatic by implementing the Error trait
// The Error trait implements both Debug + Display
// The Display trait is used when formatting a string when using println!
//          i.e println!("{}", text)
//              println!("{:?}") -> Specifies that we want to use the debugger formatter.
impl Error for ParseError {}
