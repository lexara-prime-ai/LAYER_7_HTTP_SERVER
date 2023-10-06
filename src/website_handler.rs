use crate::http::{Request, Response, StatusCode, RequestMethod};
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    // Define constructor
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            RequestMethod::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h2>Http Server</h2>".to_string())),
                "/downloads" => Response::new(StatusCode::Ok, Some("<h2>Http Server 1.1 -> :: Rust v1.72,0</h2><a href=\"www.google.com\">Downloads</a></h2>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}