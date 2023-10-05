use super::method::{RequestMethod, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::QueryString;

// Specify lifetime for buffer -> 'buf
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: RequestMethod,
}

// Buffer conversion
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // Implement custom error enum
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1 -> HTTP Request to be parsed
    // A reference to a location in memory will be passed when this method is called &'buf -> reference to buffer
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        // Read from request & assign the result to a tuple ->  (method, request)
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: RequestMethod = method.parse()?;

        // Set the mutable query_string to None
        let mut query_string = None;

        // Use if let statement to match '?' in query_string
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}


// Method to iterate over string
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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

