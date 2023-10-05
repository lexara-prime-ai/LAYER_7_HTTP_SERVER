#![allow(dead_code)]

use server::Server;
use http::Request;
use http::RequestMethod;

mod server;
mod http;



fn main() {
    // let get = http::method::RequestMethod::GET;
    // let delete = http::method::RequestMethod::DELETE;
    // let post = http::method::RequestMethod::POST;
    // let put = http::method::RequestMethod::PUT;
let mut input = String::new();

    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}


