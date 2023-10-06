use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{ParseError, Request, Response, StatusCode};

use std::convert::TryFrom;
use std::convert::TryInto;
use log::error;

// Create custom trait for handling requests
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // Anyone who implements this trait can still override the 'handle_bad_request' method even-though it has a default implementation
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    // The main constructor for a struct should be labeled new based on best practices
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    // Implement run method -> Pass in an extra argument, handler that implements the Handler trait
    pub fn run(self, mut handler: impl Handler) {
        println!("Server is listening at: {}", self.addr);

        // Bind the tcp listener to a specified address
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // Check if the response | res is an error
            // Continue | Look for the next successful connection
            // Prevent program termination by checking if the response is an error & unwrap afterwards
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // Create buffer array
                    // This size would not be recommended in production ready servers
                    // The value 1024 is only meant for testing
                    let mut buffer = [0; 1024];
                    // stream.read(&mut buffer);
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // Using from_utf8_lossy will replace invalid chars with a symbol
                            // It will not result in a crash in case byte conversion fails
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Create a slice that contains the entire byte array by omitting the upper and lower bounds
                            let response = match Request::try_from(&buffer[..]) {
                                // Format & log requests to the console
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            // If sending a response fails
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from stream | connection: {}", e)
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e)
            }
        }
    }
}