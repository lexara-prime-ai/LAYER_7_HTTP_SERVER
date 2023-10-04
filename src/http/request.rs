use super::method::RequestMethod;
use std::convert::TryFrom;

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

impl TryFrom<&[u8]> for Request {
    type Error = String;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        unimplemented!();
    }
}

