#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]


use std::fmt::format;
use server::Server;
use std::env;
use http::Request;
use http::RequestMethod;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;


fn main() {
    // Use the CARGO_MANIFEST_DIR environment varible set by cargo at run time to set the default path/working directory
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // Set custom environment variable -> PUBLIC_PATH
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    // Print default path to console
    println!("Public Path: {}", public_path);

    let server = Server::new("127.0.0.1:8000".to_string());
    server.run(WebsiteHandler::new(public_path));
}


