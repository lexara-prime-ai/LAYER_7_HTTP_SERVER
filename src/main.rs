#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]


use server::Server;
use http::Request;
use http::RequestMethod;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;


fn main() {
    // let get = http::method::RequestMethod::GET;
    // let delete = http::method::RequestMethod::DELETE;
    // let post = http::method::RequestMethod::POST;
    // let put = http::method::RequestMethod::PUT;
    let mut input = String::new();

    let server = Server::new("127.0.0.1:8000".to_string());
    server.run(WebsiteHandler);
}


