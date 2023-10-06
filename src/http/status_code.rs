use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Dereference 'self' in order to cast whatever 'self' is pointing to to a 'u16'
        // There are 2 types of values, those that live entirely on the 'stack' & 'heap'

        // e.g Stack -> i, f, &str    e.g Heap -> String

        // Strings & similar Complex types cannot implement the Copy trait
        // instead they implement the Clone trait - creates deep copies
        write!(f, "{}", *self as u16)
    }
}