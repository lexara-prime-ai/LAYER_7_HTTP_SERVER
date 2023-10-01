use super::method::RequestMethod;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: RequestMethod,
}