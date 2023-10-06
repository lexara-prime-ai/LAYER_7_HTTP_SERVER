use crate::http::{Request, Response, StatusCode, RequestMethod};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    // Define constructor
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    // Method for serving static files
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        // Prevent path traversal -> This will not work on windows machines because of how paths are defined on Windows Operating Systems.
        // Try the basic alternative at the bottom if this doesn't work for you.
        // match fs::canonicalize(path) {
        //     Ok(path) => {
        //         println!("REQUESTED FILE: {:?}\n", path);
        //         if path.starts_with(&self.public_path) {
        //             fs::read_to_string(path).ok()
        //         } else {
        //             println!("::Not Allowed::{}", file_path);
        //             None
        //         }
        //     }
        //     Err(_) => None
        // }

        // FIX FOR USERS RUNNING WINDOWS
        if path.starts_with(&self.public_path) {
            println!("REQUESTED FILE: {:?}\n", path);
            fs::read_to_string(path).ok()
        } else {
            println!("::Not Allowed::{}", file_path);
            None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            RequestMethod::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/downloads" => Response::new(StatusCode::Ok, self.read_file("downloads.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}