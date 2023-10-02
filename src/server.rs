use std::net::TcpListener;

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

    // Implement run method
    pub fn run(self) {
        println!("Server is listening at: {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
    }
}